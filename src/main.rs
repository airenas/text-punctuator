use axum::extract::DefaultBodyLimit;
use axum::middleware;
use clap::Parser;
use std::sync::Arc;
use std::time::Duration;
use text_punctuator::{
    handlers::{
        self,
        data::{self, Service},
    },
    processors::{self, metrics::Metrics},
    utils::perf::PerfLogger,
};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use tokio::signal::unix::{signal, SignalKind};

use axum::{
    routing::{get, post},
    Router,
};

/// ML POS tagger http service
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_APP_VERSION"), name = "text-punctuator-ws", about="Service for serving Sherpa onnx online punctuator", 
    long_about = None, author="Airenas V.<airenass@gmail.com>")]
struct Args {
    /// Server port
    #[arg(long, env, default_value = "8000")]
    port: u16,
    /// ONNX MODEL file
    #[arg(long, env = "ONNX_MODEL", required = true)]
    onnx_model: String,
    /// BPE VOCAB file
    #[arg(long, env = "BPE_VOCAB", required = true)]
    bpe_vocab: String,
    #[arg(long, env = "WORKERS", required = false, default_value = "2")]
    workers: i32,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    // console_subscriber::init();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();
    let args = Args::parse();
    if let Err(e) = main_int(args).await {
        log::error!("{}", e);
        return Err(e);
    }
    Ok(())
}

async fn main_int(cfg: Args) -> anyhow::Result<()> {
    let _perf_log = PerfLogger::new("loading service");
    tracing::info!("Starting Text Punctuator service");
    tracing::info!(version = env!("CARGO_APP_VERSION"));
    tracing::info!(port = cfg.port);
    tracing::info!(workers = cfg.workers);
    tracing::info!(onnx_model = cfg.onnx_model);
    tracing::info!(bpe_vocab = cfg.bpe_vocab);

    let cancel_token = CancellationToken::new();

    let ct = cancel_token.clone();

    tokio::spawn(async move {
        let mut int_stream = signal(SignalKind::interrupt()).unwrap();
        let mut term_stream = signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = int_stream.recv() => log::info!("Exit event int"),
            _ = term_stream.recv() => log::info!("Exit event term"),
        }
        tracing::debug!("sending exit event");
        ct.cancel();
        tracing::debug!("expected drop tx_close");
    });

    let restorer = processors::restorer::Restorer::new(&cfg.onnx_model, &cfg.bpe_vocab, cfg.workers)?;
    let boxed_restorer: Box<dyn data::Processor + Send + Sync> = Box::new(restorer);

    let srv = Arc::new(RwLock::new(Service {
        calls: 0,
        onnx: boxed_restorer,
    }));

    let metrics = Metrics::new()?;

    let metrics_cl = metrics.clone();

    let helper_router = axum::Router::new().route("/live", get(handlers::live::handler));

    let main_router = Router::new()
        .route("/punctuate", post(handlers::punctuate::handler))
        .with_state(srv.clone())
        .layer(middleware::from_fn(move |req, next| {
            let mc = metrics_cl.clone();
            async move { mc.observe(req, next).await }
        }));

    let app = Router::new()
        .merge(helper_router)
        .merge(main_router)
        .route("/metrics", get(handlers::metrics::handler))
        .layer((
            DefaultBodyLimit::max(1024 * 1024),
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    std::mem::drop(_perf_log);
    tracing::info!(port = cfg.port, "serving ...");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", cfg.port)).await?;

    let handle = axum_server::Handle::new();
    let shutdown_future = shutdown_signal_handle(handle.clone(), cancel_token.clone());
    tokio::spawn(shutdown_future);

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            cancel_token.cancelled().await;
        })
        .await?;

    tracing::info!("Bye");
    Ok(())
}

async fn shutdown_signal_handle(handle: axum_server::Handle, cancel_token: CancellationToken) {
    cancel_token.cancelled().await;
    tracing::trace!("Received termination signal shutting down");
    handle.graceful_shutdown(Some(Duration::from_secs(10)));
}

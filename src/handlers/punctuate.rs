use std::sync::Arc;

use crate::{
    handlers::data::{PunctuateParams, PunctuateRequest, PunctuateResponse, Service},
    utils::perf::PerfLogger,
};
use anyhow::Context;
use axum::{
    debug_handler,
    extract::{self, Query, State},
    Json,
};
use tokio::sync::RwLock;

use super::data::ApiResult;

#[debug_handler]
pub async fn handler(
    State(srv_wrap): State<Arc<RwLock<Service>>>,
    Query(_params): Query<PunctuateParams>,
    Json(input): Json<PunctuateRequest>,
) -> ApiResult<extract::Json<PunctuateResponse>> {
    let _perf_log = PerfLogger::new("punctuation handler");

    let srv = srv_wrap.read().await;
    let res = srv.onnx.process(&input.text).await.context("punctuation")?;
    let res = PunctuateResponse { text: res };
    Ok(Json(res))
}

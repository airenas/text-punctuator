use std::sync::Arc;

use crate::{
    handlers::data::{PunctuationParams, PunctuationRequest, PunctuationResponse, Service},
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
    Query(_params): Query<PunctuationParams>,
    Json(input): Json<PunctuationRequest>,
) -> ApiResult<extract::Json<PunctuationResponse>> {
    let _perf_log = PerfLogger::new("punctuation handler");

    let srv = srv_wrap.read().await;
    let res = srv.onnx.process(&input.text).await.context("punctuation")?;
    let res = PunctuationResponse {
        text: input.text,
        result: res,
    };
    Ok(Json(res))
}

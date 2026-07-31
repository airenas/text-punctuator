use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::error::ApiError;

#[async_trait]
pub trait Processor {
    async fn process(&self, text: &str) -> anyhow::Result<String>;
}

pub struct Service {
    pub onnx: Box<dyn Processor + Send + Sync>,
    pub calls: u32,
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Serialize, Clone)]
pub struct LiveResponse {
    pub status: bool,
    pub version: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PunctuationResponse {
    pub text: String,
    pub result: String,
}

#[derive(Deserialize)]
pub struct PunctuationRequest {
    pub text: String,
}

#[derive(Deserialize)]
pub struct PunctuationParams {
    pub debug: Option<String>,
}

impl PunctuationParams {
    pub fn debug(&self) -> bool {
        self.debug.is_some()
    }
}

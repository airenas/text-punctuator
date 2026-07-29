use anyhow::Ok;
use async_trait::async_trait;
use sherpa_onnx::{OnlinePunctuation, OnlinePunctuationConfig, OnlinePunctuationModelConfig};

use crate::handlers::data::Processor;
use crate::utils::perf::PerfLogger;

pub struct Restorer {
    model: OnlinePunctuation,
}

impl Restorer {
    pub fn new(onnx_model: &str, bpe_vocab: &str, workers: i32) -> anyhow::Result<Restorer> {
        let _perf_log = PerfLogger::new("punctuation model loader");
        let config = OnlinePunctuationConfig {
            model: OnlinePunctuationModelConfig {
                cnn_bilstm: Some(onnx_model.to_string()),
                bpe_vocab: Some(bpe_vocab.to_string()),
                num_threads: workers,
                provider: Some("cpu".to_string()),
                debug: true
            },
        };
        let model = OnlinePunctuation::create(&config)
            .ok_or_else(|| anyhow::anyhow!("Failed to create OnlinePunctuation"))?;

        let res = Restorer { model };
        Ok(res)
    }
}

#[async_trait]
impl Processor for Restorer {
    async fn process(&self, text: &str) -> anyhow::Result<String> {
        let _perf_log = PerfLogger::new("restorer mapper");
        let res = self
            .model
            .add_punctuation(text)
            .ok_or_else(|| anyhow::anyhow!("Failed to add punctuation"))?;
        Ok(res)
    }
}

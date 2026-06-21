use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use finance_assistant_app::{
    errors::AppError,
    ports::ai_client_port::{AiClientPort, ExtractedInvoiceFields},
};

pub struct OpenAiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            model: "gpt-4o".to_string(),
        }
    }
}

#[async_trait]
impl AiClientPort for OpenAiClient {
    async fn extract_invoice_fields(
        &self,
        _document_bytes: &[u8],
        _content_type: &str,
    ) -> Result<ExtractedInvoiceFields, AppError> {
        // TODO: implement actual OpenAI vision API call for invoice OCR
        Err(AppError::ExternalService {
            service: "OpenAI".to_string(),
            message: "Not yet implemented".to_string(),
        })
    }

    async fn answer_financial_query(
        &self,
        context: &str,
        question: &str,
    ) -> Result<String, AppError> {
        let body = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are an AI Finance & Accounting Assistant. Answer based only on the provided context. Use Indonesian."
                },
                {
                    "role": "user",
                    "content": format!("Context:\n{}\n\nQuestion: {}", context, question)
                }
            ],
            "max_tokens": 1024,
            "temperature": 0.2
        });

        let resp = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "OpenAI".to_string(),
                message: e.to_string(),
            })?;

        let json: serde_json::Value = resp.json().await.map_err(|e| AppError::ExternalService {
            service: "OpenAI".to_string(),
            message: e.to_string(),
        })?;

        let answer = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Maaf, tidak dapat memproses permintaan saat ini.")
            .to_string();

        Ok(answer)
    }
}

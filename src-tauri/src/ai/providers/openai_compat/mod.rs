mod adapter;
mod models;
mod registry;

use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::ai::error::AIError;
use crate::ai::AIProvider;

use adapter::PreparedBody;
use registry::OpenAICompatModelRegistry;

const DEFAULT_BASE_URL: &str = "https://api.uniapi.io";

pub struct OpenAICompatProvider {
	client: Client,
	api_key: Arc<RwLock<Option<String>>>,
	base_url: String,
	model_registry: OpenAICompatModelRegistry,
}

#[derive(Debug, Deserialize)]
struct ImagesResponse {
	data: Option<Vec<ImageItem>>,
}

#[derive(Debug, Deserialize)]
struct ImageItem {
	b64_json: Option<String>,
	url: Option<String>,
}

impl OpenAICompatProvider {
	pub fn new() -> Self {
		Self {
			client: Client::new(),
			api_key: Arc::new(RwLock::new(None)),
			base_url: DEFAULT_BASE_URL.to_string(),
			model_registry: OpenAICompatModelRegistry::new(),
		}
	}

	fn extract_image_source(body: &ImagesResponse) -> Option<String> {
		let items = body.data.as_ref()?;
		for item in items {
			if let Some(b64) = item.b64_json.as_ref().filter(|value| !value.trim().is_empty()) {
				return Some(format!("data:image/png;base64,{}", b64));
			}
			if let Some(url) = item.url.as_ref().filter(|value| !value.trim().is_empty()) {
				return Some(url.clone());
			}
		}
		None
	}
}

impl Default for OpenAICompatProvider {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait::async_trait]
impl AIProvider for OpenAICompatProvider {
	fn name(&self) -> &str {
		"openai-compat"
	}

	fn supports_model(&self, model: &str) -> bool {
		self.model_registry.supports(model)
	}

	fn list_models(&self) -> Vec<String> {
		self.model_registry.list_models()
	}

	async fn set_api_key(&self, api_key: String) -> Result<(), AIError> {
		let mut key = self.api_key.write().await;
		*key = Some(api_key);
		Ok(())
	}

	async fn generate(&self, request: crate::ai::GenerateRequest) -> Result<String, AIError> {
		let key = self.api_key.read().await;
		let api_key = key
			.as_ref()
			.ok_or_else(|| AIError::InvalidRequest("API key not set".to_string()))?;

		let adapter = self
			.model_registry
			.resolve(&request.model)
			.ok_or_else(|| AIError::ModelNotSupported(request.model.clone()))?;

		let prepared = adapter.build_request(&request, &self.base_url)?;

		info!("[OpenAI-Compat Request] {}", prepared.summary);
		info!("[OpenAI-Compat API] URL: {}", prepared.endpoint);

		let builder = self
			.client
			.post(&prepared.endpoint)
			.header("Authorization", format!("Bearer {}", api_key));

		let response = match prepared.body {
			PreparedBody::Json(value) => builder
				.header("Content-Type", "application/json")
				.json(&value)
				.send()
				.await?,
			PreparedBody::Multipart(form) => builder.multipart(form).send().await?,
		};

		let status = response.status();
		let raw_response = response.text().await.unwrap_or_default();
		if !status.is_success() {
			return Err(AIError::Provider(format!(
				"OpenAI-Compat API error {}: {}",
				status, raw_response
			)));
		}

		let body = serde_json::from_str::<ImagesResponse>(&raw_response).map_err(|err| {
			AIError::Provider(format!(
				"OpenAI-Compat invalid JSON response: {}; raw={}",
				err,
				truncate_for_log(&raw_response, 512)
			))
		})?;

		if let Some(image_source) = Self::extract_image_source(&body) {
			info!("Generated image source (prefix): {}", truncate_for_log(&image_source, 64));
			Ok(image_source)
		} else {
			let fallback = serde_json::from_str::<Value>(&raw_response)
				.ok()
				.map(|value| value.to_string())
				.unwrap_or(raw_response);
			Err(AIError::Provider(format!(
				"OpenAI-Compat response missing image data: {}",
				truncate_for_log(&fallback, 256)
			)))
		}
	}
}

fn truncate_for_log(input: &str, max_chars: usize) -> String {
	if input.chars().count() <= max_chars {
		return input.to_string();
	}
	input.chars().take(max_chars).collect::<String>()
}

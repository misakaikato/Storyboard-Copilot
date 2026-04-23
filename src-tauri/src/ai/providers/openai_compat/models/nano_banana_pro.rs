use super::super::adapter::OpenAICompatModelAdapter;

pub struct NanoBananaProAdapter;

impl NanoBananaProAdapter {
	pub fn new() -> Self {
		Self
	}
}

impl Default for NanoBananaProAdapter {
	fn default() -> Self {
		Self::new()
	}
}

impl OpenAICompatModelAdapter for NanoBananaProAdapter {
	fn model_aliases(&self) -> &'static [&'static str] {
		&["openai-compat/nano-banana-pro"]
	}

	fn api_model_name(&self) -> &'static str {
		"google/nano-banana-pro"
	}
}

inventory::submit! {
	crate::ai::providers::openai_compat::models::RegisteredOpenAICompatModel {
		build: || Box::new(NanoBananaProAdapter::new()),
	}
}

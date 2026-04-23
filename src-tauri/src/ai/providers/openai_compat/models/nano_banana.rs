use super::super::adapter::OpenAICompatModelAdapter;

pub struct NanoBananaAdapter;

impl NanoBananaAdapter {
	pub fn new() -> Self {
		Self
	}
}

impl Default for NanoBananaAdapter {
	fn default() -> Self {
		Self::new()
	}
}

impl OpenAICompatModelAdapter for NanoBananaAdapter {
	fn model_aliases(&self) -> &'static [&'static str] {
		&["openai-compat/nano-banana"]
	}

	fn api_model_name(&self) -> &'static str {
		"google/nano-banana"
	}
}

inventory::submit! {
	crate::ai::providers::openai_compat::models::RegisteredOpenAICompatModel {
		build: || Box::new(NanoBananaAdapter::new()),
	}
}

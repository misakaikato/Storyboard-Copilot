use super::super::adapter::OpenAICompatModelAdapter;

pub struct NanoBanana2Adapter;

impl NanoBanana2Adapter {
	pub fn new() -> Self {
		Self
	}
}

impl Default for NanoBanana2Adapter {
	fn default() -> Self {
		Self::new()
	}
}

impl OpenAICompatModelAdapter for NanoBanana2Adapter {
	fn model_aliases(&self) -> &'static [&'static str] {
		&["openai-compat/nano-banana-2"]
	}

	fn api_model_name(&self) -> &'static str {
		"google/nano-banana-2"
	}
}

inventory::submit! {
	crate::ai::providers::openai_compat::models::RegisteredOpenAICompatModel {
		build: || Box::new(NanoBanana2Adapter::new()),
	}
}

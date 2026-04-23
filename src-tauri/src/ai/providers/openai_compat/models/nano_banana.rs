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
		&[
			"openai-compat/gemini-2.5-flash-image",
			"gemini-2.5-flash-image",
		]
	}

	fn api_model_name(&self) -> &'static str {
		"gemini-2.5-flash-image"
	}
}

inventory::submit! {
	crate::ai::providers::openai_compat::models::RegisteredOpenAICompatModel {
		build: || Box::new(NanoBananaAdapter::new()),
	}
}

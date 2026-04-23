use super::adapter::OpenAICompatModelAdapter;

automod::dir!("src/ai/providers/openai_compat/models");

pub struct RegisteredOpenAICompatModel {
	pub build: fn() -> Box<dyn OpenAICompatModelAdapter>,
}

inventory::collect!(RegisteredOpenAICompatModel);

pub fn collect_adapters() -> Vec<Box<dyn OpenAICompatModelAdapter>> {
	inventory::iter::<RegisteredOpenAICompatModel>
		.into_iter()
		.map(|entry| (entry.build)())
		.collect()
}

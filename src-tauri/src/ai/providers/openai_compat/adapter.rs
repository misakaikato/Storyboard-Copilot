use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::multipart::{Form, Part};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::ai::error::AIError;
use crate::ai::GenerateRequest;

const MAX_REFERENCE_IMAGES: usize = 16;
const SUPPORTED_SIZES: &[&str] = &["1024x1024", "1536x1024", "1024x1536", "auto"];

pub enum PreparedBody {
	Json(Value),
	Multipart(Form),
}

pub struct PreparedRequest {
	pub endpoint: String,
	pub body: PreparedBody,
	pub summary: String,
}

pub trait OpenAICompatModelAdapter: Send + Sync {
	fn model_aliases(&self) -> &'static [&'static str];

	fn api_model_name(&self) -> &'static str;

	fn canonical_model(&self) -> &'static str {
		self.model_aliases()
			.iter()
			.find(|model| model.contains('/'))
			.copied()
			.or_else(|| self.model_aliases().first().copied())
			.unwrap_or("unknown")
	}

	fn matches(&self, model: &str) -> bool {
		self.model_aliases().iter().any(|alias| alias == &model)
	}

	fn build_request(
		&self,
		request: &GenerateRequest,
		base_url: &str,
	) -> Result<PreparedRequest, AIError> {
		build_openai_image_request(self.api_model_name(), self.canonical_model(), request, base_url)
	}
}

pub fn build_openai_image_request(
	api_model_name: &str,
	display_model_id: &str,
	request: &GenerateRequest,
	base_url: &str,
) -> Result<PreparedRequest, AIError> {
	let has_reference_images = request
		.reference_images
		.as_ref()
		.map(|images| !images.is_empty())
		.unwrap_or(false);
	let size = sanitize_size(&request.size);

	if has_reference_images {
		let reference_images = request.reference_images.as_ref().unwrap();
		let capped: Vec<&String> = reference_images.iter().take(MAX_REFERENCE_IMAGES).collect();

		let mut form = Form::new()
			.text("model", api_model_name.to_string())
			.text("prompt", request.prompt.clone())
			.text("n", "1");
		if let Some(size_value) = size.as_ref() {
			form = form.text("size", size_value.clone());
		}

		for (index, source) in capped.iter().enumerate() {
			let (bytes, mime, extension) = resolve_reference_image_bytes(source).map_err(|err| {
				AIError::InvalidRequest(format!(
					"Failed to read reference image #{} for OpenAI-Compat edit: {}",
					index + 1,
					err
				))
			})?;
			let file_name = format!("reference-{}.{}", index + 1, extension);
			let part = Part::bytes(bytes)
				.file_name(file_name)
				.mime_str(&mime)
				.map_err(|err| {
					AIError::InvalidRequest(format!(
						"Invalid mime type for reference image #{}: {}",
						index + 1,
						err
					))
				})?;
			form = form.part("image[]", part);
		}

		let summary = format!(
			"model: {}, mode: edit, images: {}, size: {}, prompt: {}",
			display_model_id,
			capped.len(),
			size.as_deref().unwrap_or("default"),
			truncate_for_log(&request.prompt, 100)
		);

		Ok(PreparedRequest {
			endpoint: format!("{}/v1/images/edits", base_url),
			body: PreparedBody::Multipart(form),
			summary,
		})
	} else {
		let mut body = json!({
			"model": api_model_name,
			"prompt": request.prompt,
			"n": 1,
		});
		if let Some(size_value) = size.as_ref() {
			body["size"] = json!(size_value);
		}

		let summary = format!(
			"model: {}, mode: generate, size: {}, prompt: {}",
			display_model_id,
			size.as_deref().unwrap_or("default"),
			truncate_for_log(&request.prompt, 100)
		);

		Ok(PreparedRequest {
			endpoint: format!("{}/v1/images/generations", base_url),
			body: PreparedBody::Json(body),
			summary,
		})
	}
}

fn sanitize_size(size: &str) -> Option<String> {
	let trimmed = size.trim();
	if trimmed.is_empty() {
		return None;
	}
	if SUPPORTED_SIZES
		.iter()
		.any(|candidate| candidate.eq_ignore_ascii_case(trimmed))
	{
		return Some(trimmed.to_lowercase());
	}
	None
}

fn decode_file_url_path(value: &str) -> String {
	let raw = value.trim_start_matches("file://");
	let decoded = urlencoding::decode(raw)
		.map(|result| result.into_owned())
		.unwrap_or_else(|_| raw.to_string());
	let normalized = if decoded.starts_with('/')
		&& decoded.len() > 2
		&& decoded.as_bytes().get(2) == Some(&b':')
	{
		&decoded[1..]
	} else {
		&decoded
	};
	normalized.to_string()
}

fn resolve_reference_image_bytes(source: &str) -> Result<(Vec<u8>, String, String), String> {
	let trimmed = source.trim();
	if trimmed.is_empty() {
		return Err("source is empty".to_string());
	}

	if let Some((meta, payload)) = trimmed.split_once(',') {
		if meta.starts_with("data:") && meta.ends_with(";base64") && !payload.is_empty() {
			let bytes = STANDARD
				.decode(payload)
				.map_err(|err| format!("invalid data-url base64 payload: {}", err))?;
			let mime = meta
				.strip_prefix("data:")
				.and_then(|rest| rest.split(';').next())
				.unwrap_or("image/png")
				.to_string();
			let extension = mime_to_extension(&mime);
			return Ok((bytes, mime, extension));
		}
	}

	let path = if trimmed.starts_with("file://") {
		PathBuf::from(decode_file_url_path(trimmed))
	} else {
		PathBuf::from(trimmed)
	};

	let bytes = std::fs::read(&path).map_err(|err| {
		format!(
			"failed to read path \"{}\": {}",
			path.to_string_lossy(),
			err
		)
	})?;
	let extension = path
		.extension()
		.and_then(|ext| ext.to_str())
		.map(|ext| ext.to_ascii_lowercase())
		.unwrap_or_else(|| "png".to_string());
	let mime = extension_to_mime(&extension).to_string();
	Ok((bytes, mime, extension))
}

fn mime_to_extension(mime: &str) -> String {
	match mime {
		"image/png" => "png".to_string(),
		"image/jpeg" | "image/jpg" => "jpg".to_string(),
		"image/webp" => "webp".to_string(),
		_ => "png".to_string(),
	}
}

fn extension_to_mime(extension: &str) -> &'static str {
	match extension {
		"jpg" | "jpeg" => "image/jpeg",
		"webp" => "image/webp",
		_ => "image/png",
	}
}

fn truncate_for_log(input: &str, max_chars: usize) -> String {
	if input.chars().count() <= max_chars {
		return input.to_string();
	}
	input.chars().take(max_chars).collect::<String>()
}

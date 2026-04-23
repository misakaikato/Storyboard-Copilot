import type { ImageModelDefinition } from '../../types';

export const OPENAI_COMPAT_NANO_BANANA_PRO_MODEL_ID = 'openai-compat/nano-banana-pro';

export const imageModel: ImageModelDefinition = {
	id: OPENAI_COMPAT_NANO_BANANA_PRO_MODEL_ID,
	mediaType: 'image',
	displayName: 'Nano Banana Pro',
	providerId: 'openai-compat',
	description: 'OpenAI 兼容格式 · google/nano-banana-pro',
	eta: '2min',
	expectedDurationMs: 120000,
	defaultAspectRatio: '1:1',
	defaultResolution: '1024x1024',
	aspectRatios: [
		{ value: '1:1', label: '1:1' },
		{ value: '3:2', label: '3:2' },
		{ value: '2:3', label: '2:3' },
		{ value: 'auto', label: 'Auto' },
	],
	resolutions: [
		{ value: '1024x1024', label: '1024×1024' },
		{ value: '1536x1024', label: '1536×1024' },
		{ value: '1024x1536', label: '1024×1536' },
		{ value: 'auto', label: 'Auto' },
	],
	resolveRequest: ({ referenceImageCount }) => ({
		requestModel: OPENAI_COMPAT_NANO_BANANA_PRO_MODEL_ID,
		modeLabel: referenceImageCount > 0 ? '编辑模式' : '生成模式',
	}),
};

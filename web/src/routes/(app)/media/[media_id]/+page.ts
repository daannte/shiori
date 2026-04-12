import { createClient, type operations } from '@shiori/api-client';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

type MediaResponse = operations['get_media']['responses']['200']['content']['application/json'];

export const load: PageLoad = async ({ fetch, params, depends }) => {
	depends('media:page');

	const client = createClient({ fetch });

	const mediaId = parseInt(params.media_id);
	if (isNaN(mediaId)) {
		throw error(400, { message: 'Invalid media id' });
	}

	const media = await loadMedia(client, mediaId);

	return media;
};

function loadMediaError(status: number): never {
	error(status, { message: 'Failed to load media item', tryAgain: true });
}

async function loadMedia(
	client: ReturnType<typeof createClient>,
	id: number
): Promise<MediaResponse> {
	const res = await client.GET('/api/v1/media/{id}', { params: { path: { id } } }).catch(() => {
		loadMediaError(504);
	});

	if (res.error || !res.data) {
		loadMediaError(res.response.status);
	}

	return res.data;
}

import { createClient, type operations } from '@shiori/api-client';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

type LibraryMediaResponse =
	operations['list_library_media']['responses']['200']['content']['application/json'];

export const load: PageLoad = async ({ fetch, params, depends }) => {
	depends('libraries:media');

	const client = createClient({ fetch });

	const libraryId = parseInt(params.library_id);
	if (isNaN(libraryId)) {
		throw error(400, { message: 'Invalid library id' });
	}

	const libraryMedia = await loadLibraryMedia(client, libraryId);

	return { media: libraryMedia, libraryId };
};

function loadLibraryMediaError(status: number): never {
	error(status, { message: 'Failed to load library media', tryAgain: true });
}

async function loadLibraryMedia(
	client: ReturnType<typeof createClient>,
	id: number
): Promise<LibraryMediaResponse> {
	const res = await client
		.GET('/api/v1/libraries/{id}/media', { params: { path: { id } } })
		.catch(() => {
			loadLibraryMediaError(504);
		});

	if (res.error || !res.data) {
		loadLibraryMediaError(res.response.status);
	}

	return res.data;
}

import { createClient, type operations } from '@shiori/api-client';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

type Directories =
	operations['list_directories']['responses']['200']['content']['application/json'];

export const load: PageLoad = async ({ fetch }) => {
	const client = createClient({ fetch });

	const directories = await loadCreate(client);

	return directories;
};

function loadCreateError(status: number): never {
	error(status, { message: 'Failed to load library media', tryAgain: true });
}

async function loadCreate(client: ReturnType<typeof createClient>): Promise<Directories> {
	const res = await client
		.POST('/api/v1/filesystem/directories/list', { body: { path: '' } })
		.catch(() => {
			loadCreateError(504);
		});

	if (res.error || !res.data) {
		loadCreateError(res.response.status);
	}

	return res.data;
}

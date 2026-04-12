import { createClient, type operations } from '@shiori/api-client';
import type { PageLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { loadUser } from '$lib/session.svelte';
import { resolve } from '$app/paths';

type MetaResponse = operations['meta']['responses']['200']['content']['application/json'];

export const load: PageLoad = async ({ fetch }) => {
	const client = createClient({ fetch });

	const user = await loadUser(client);

	if (user) {
		redirect(303, resolve('/'));
	}

	const meta = await loadMeta(client);

	return meta;
};

function loadMetaError(status: number): never {
	error(status, { message: 'Failed to load system metadata', tryAgain: true });
}

async function loadMeta(client: ReturnType<typeof createClient>): Promise<MetaResponse> {
	const res = await client.GET('/api/v1/meta').catch(() => {
		loadMetaError(504);
	});

	if (res.error || !res.data) {
		loadMetaError(res.response.status);
	}

	return res.data;
}

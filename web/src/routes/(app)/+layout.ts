import type { operations } from '@shiori/api-client';
import type { LayoutLoad } from './$types';

import { resolve } from '$app/paths';
import { createClient } from '@shiori/api-client';
import { error, redirect } from '@sveltejs/kit';

import { loadUser } from '$lib/session.svelte';

type LibrariesResponse =
	operations['list_libraries']['responses']['200']['content']['application/json'];

export const load: LayoutLoad = async ({ fetch, depends }) => {
	depends('libraries:create');

	const client = createClient({ fetch });

	const user = await loadUser(client);

	if (!user) {
		redirect(303, resolve('/auth'));
	}

	const libraries = await loadLibraries(client);

	return {
		user,
		libraries
	};
};

function loadLibrariesError(status: number): never {
	error(status, { message: 'Failed to load libraries', tryAgain: true });
}

async function loadLibraries(client: ReturnType<typeof createClient>): Promise<LibrariesResponse> {
	const res = await client.GET('/api/v1/libraries').catch(() => {
		loadLibrariesError(504);
	});

	if (res.error || !res.data) {
		loadLibrariesError(res.response.status);
	}

	return res.data;
}

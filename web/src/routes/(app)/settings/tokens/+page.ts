import { createClient, type operations } from '@shiori/api-client';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';

type TokensResponse = operations['list_tokens']['responses']['200']['content']['application/json'];

export const load: PageLoad = async ({ fetch, depends }) => {
	depends('tokens:load');

	const client = createClient({ fetch });

	const tokens = await loadTokens(client);

	return { tokens: tokens };
};

function loadTokensError(status: number): never {
	error(status, { message: 'Failed to load tokens', tryAgain: true });
}

async function loadTokens(client: ReturnType<typeof createClient>): Promise<TokensResponse> {
	const res = await client.GET('/api/v1/tokens').catch(() => {
		loadTokensError(504);
	});

	if (res.error || !res.data) {
		loadTokensError(res.response.status);
	}

	return res.data;
}

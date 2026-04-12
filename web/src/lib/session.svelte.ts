import { createClient, type operations } from '@shiori/api-client';

type User = operations['me']['responses']['200']['content']['application/json'];

export async function loadUser(client: ReturnType<typeof createClient>): Promise<User | null> {
	try {
		const res = await client.GET('/api/v1/auth/me');
		if (res.error || !res.data) return null;
		return res.data;
	} catch {
		return null;
	}
}

<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import { loginSchema, registerSchema, type LoginBody, type RegisterBody } from './schema';
	import { createClient } from '@shiori/api-client';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let client = createClient({ fetch });

	let { data } = $props();

	let isLoading = $state(false);

	async function login(body: LoginBody) {
		let res = await client.POST('/api/v1/auth/login', { body });
		if (!res.data || res.error) throw new Error('Login failed');
		goto(resolve('/'), { replaceState: true, invalidateAll: true });
	}

	async function register(body: RegisterBody) {
		let res = await client.POST('/api/v1/auth/register', { body });

		if (!res.data || res.error) {
			throw new Error('Register failed');
		}
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		isLoading = true;

		try {
			const formData = new FormData(e.currentTarget as HTMLFormElement);

			const values = {
				username: formData.get('username'),
				password: formData.get('password'),
				confirm: formData.get('confirm')
			};

			const schema = data.initialized ? loginSchema : registerSchema;

			const zRes = schema.safeParse(values);

			if (!zRes.success) {
				isLoading = false;
				return;
			}

			if (data.initialized) {
				await login(zRes.data as LoginBody);
			} else {
				await register(zRes.data as RegisterBody);

				await login({
					username: zRes.data.username,
					password: zRes.data.password
				});
			}
		} catch (e) {
			console.error('Auth Error:', e);
		} finally {
			isLoading = false;
		}
	}
</script>

<form class="flex min-h-screen items-center justify-center" onsubmit={handleSubmit}>
	<div class="grid max-w-4xl gap-4">
		<div class="grid gap-1">
			<Label for="username">Username</Label>
			<Input id="username" name="username" placeholder="Username" />
		</div>

		<div class="grid gap-1">
			<Label for="password">Password</Label>
			<Input id="password" type="password" name="password" placeholder="Password" />
		</div>

		{#if !data.initialized}
			<div class="grid gap-1">
				<Label for="confirm">Confirm Password</Label>
				<Input id="confirm" type="password" name="confirm" placeholder="Confirm Password" />
			</div>
		{/if}

		<Button type="submit" disabled={isLoading}>
			{#if isLoading}
				<LoaderCircle class="animate-spin" />
			{:else}
				{data.initialized ? 'Login' : 'Register'}
			{/if}
		</Button>
	</div>
</form>

<script lang="ts">
	import { goto } from '$app/navigation';

	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';

	import { librarySchema } from './schema';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';

	let name = $state('');
	let path = $state('');

	let loading = $state(false);
	let errors = $state<Record<string, string>>({});

	async function handleSubmit() {
		errors = {};

		const result = librarySchema.safeParse({ name, path });

		if (!result.success) {
			for (const err of result.error.issues) {
				const key = err.path[0] as string;
				errors[key] = err.message;
			}
			return;
		}

		loading = true;

		try {
			const res = await fetch(`/api/v1/libraries`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(result.data)
			});
			if (!res.ok) throw new Error('Failed to create library');

			name = '';
			path = '';
			goto('/');
		} catch (error) {
			console.error(error);
		} finally {
			loading = false;
		}
	}
</script>

<form class="flex min-h-screen items-center justify-center" onsubmit={handleSubmit}>
	<div class="grid max-w-4xl gap-4">
		<div class="grid gap-1">
			<Label for="name">Name</Label>
			<Input id="name" name="name" placeholder="Novels" bind:value={name} />
			{#if errors.name}
				<p class="text-sm text-destructive">{errors.name}</p>
			{/if}
		</div>
		<div class="grid gap-1">
			<Label for="path">Path</Label>
			<Input id="path" name="path" placeholder="/data/books/novels" bind:value={path} />
			{#if errors.path}
				<p class="text-sm text-destructive">{errors.path}</p>
			{/if}
		</div>

		<Button type="submit" disabled={loading}>
			{#if loading}
				<LoaderCircle class="animate-spin" />
			{:else}
				Create Library
			{/if}
		</Button>
	</div>
</form>

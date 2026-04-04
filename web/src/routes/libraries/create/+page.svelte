<script lang="ts">
	import { goto } from '$app/navigation';
	import { createClient, type components } from '@shiori/api-client';
	import { librarySchema } from './schema';

	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import FolderPicker from '$lib/components/folder-picker/folder-picker.svelte';

	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Folder from '@lucide/svelte/icons/folder';

	type Directory = components['schemas']['EncodableDirectory'];

	let client = createClient({ fetch });

	let { data } = $props();
	let dirs = $derived(data.directories);
	let parent = $derived(data.parent);

	let name = $state('');
	let directory = $state<Directory | undefined>();
	let isFolderPickerOpen = $state(false);

	let loading = $state(false);
	let errors = $state<Record<string, string>>({});

	async function handleSubmit() {
		errors = {};

		if (!directory) return;

		const result = librarySchema.safeParse({ name, path: directory.path });

		if (!result.success) {
			for (const err of result.error.issues) {
				const key = err.path[0] as string;
				errors[key] = err.message;
			}
			return;
		}

		loading = true;

		try {
			const res = await client.POST('/api/v1/libraries', {
				body: result.data
			});
			if (!res.response.ok) throw new Error('Failed to create library');
			goto('/', { invalidate: ['libraries:create'] });
		} catch (error) {
			console.error(error);
		} finally {
			loading = false;
		}
	}

	async function updateDirs() {
		if (!directory || !directory.has_children) return;
		try {
			const res = await client.POST('/api/v1/filesystem/directories/list', {
				body: { path: directory.path }
			});
			if (!res.data || res.error) throw new Error('Failed to update directories');

			dirs = res.data.directories;
			parent = res.data.parent;
		} catch (error) {
			console.error(error);
		}
	}

	$effect(() => {
		updateDirs();
	});
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
			<Label>Add Path</Label>

			<Button size="icon" variant="outline" onclick={() => (isFolderPickerOpen = true)}>
				<Folder />
			</Button>

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

<FolderPicker {dirs} {parent} bind:isOpen={isFolderPickerOpen} bind:directory />

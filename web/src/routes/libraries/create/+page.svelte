<script lang="ts">
	import { goto } from '$app/navigation';
	import { createClient } from '@shiori/api-client';

	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import FolderPicker from '$lib/components/folder-picker/folder-picker.svelte';

	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Folder from '@lucide/svelte/icons/folder';
	import { resolve } from '$app/paths';

	let client = createClient({ fetch });

	let { data } = $props();
	let dirs = $derived(data.directories);
	let parent = $derived(data.parent);

	let path = $state<string | null>(null);
	let selectedPath = $state<string | null>(null);

	let name = $state('');
	let isFolderPickerOpen = $state(false);

	let loading = $state(false);

	async function handleSubmit() {
		if (!selectedPath || !name) return;

		loading = true;

		try {
			const res = await client.POST('/api/v1/libraries', {
				body: { path: selectedPath, name }
			});
			if (!res.response.ok || !res.data) throw new Error('Failed to create library');
			goto(
				resolve('/libraries/[library_id]/media', {
					library_id: res.data.id.toString()
				}),
				{
					invalidate: ['libraries:create']
				}
			);
		} catch (error) {
			console.error(error);
		} finally {
			loading = false;
		}
	}

	async function updateDirs() {
		if (!isFolderPickerOpen) return;

		try {
			const res = await client.POST('/api/v1/filesystem/directories/list', {
				body: { path: path ? path : '' }
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
		</div>
		<div class="flex items-center gap-2 rounded-md border border-border bg-background p-2">
			<div class="flex-1">
				<p class="mt-1 truncate text-sm">
					{selectedPath ? `/${selectedPath}` : 'No path selected'}
				</p>
			</div>

			<Button size="icon" variant="outline" onclick={() => (isFolderPickerOpen = true)}>
				<Folder />
			</Button>
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

<FolderPicker {dirs} {parent} bind:isOpen={isFolderPickerOpen} bind:path bind:selectedPath />

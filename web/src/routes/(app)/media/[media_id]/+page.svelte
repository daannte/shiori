<script lang="ts">
	import type { components, operations } from '@shiori/api-client';

	import { goto, invalidate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Check from '@lucide/svelte/icons/check';
	import Download from '@lucide/svelte/icons/download';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import { createClient, get_cover_url } from '@shiori/api-client';
	import { Button, Progress } from '@shiori/components';
	import { format } from 'date-fns';

	import Dialog from '$lib/components/dialog.svelte';
	import MetadataDialog from '$lib/components/metadata/metadata-dialog.svelte';

	type PatchMetadata = components['schemas']['PatchMetadata'];
	type MetadataSearch =
		operations['get_book_metadata']['responses']['200']['content']['application/json'];

	let { data } = $props();

	let client = createClient({ fetch });
	let isMetadataOpen = $state(false);
	let isDeleteOpen = $state(false);

	let metadataSearch = $state.raw<MetadataSearch | undefined>();

	let metadataArr = $derived.by(() =>
		data.metadata
			? Object.entries(data.metadata).filter(
					([key]) => !['genres', 'description', 'authors'].includes(key)
				)
			: []
	);

	function formatValue(key: string, value: string | string[] | null) {
		if (!value) return 'Unknown';

		if (key === 'published' && typeof value === 'string') {
			return format(value, 'PPP');
		}

		return value;
	}

	function labelize(key: string) {
		if (key.toLowerCase() === 'isbn') return 'ISBN';
		return key.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	async function saveMetadata() {
		if (!metadataSearch) return;

		let patch: PatchMetadata = {
			...metadataSearch,
			authors: metadataSearch.authors.length ? metadataSearch.authors : null,
			genres: metadataSearch.genres.length ? metadataSearch.genres : null
		};

		try {
			let res = await client.PATCH('/api/v1/media/{id}', {
				params: { path: { id: data.id } },
				body: { name: metadataSearch.title, cover_url: metadataSearch.cover_url, metadata: patch }
			});
			if (!res.response.ok) throw new Error('Not good');
			metadataSearch = undefined;
			invalidate('media:page');
		} catch (e) {
			console.error('Failed to save metadata: ', e);
		}
	}

	async function handleDelete() {
		try {
			let res = await client.DELETE('/api/v1/media/{id}', { params: { path: { id: data.id } } });
			if (res.error || res.data) throw new Error();
			goto(
				resolve('/(app)/libraries/[library_id]/media', { library_id: data.library_id.toString() })
			);
		} catch (e) {
			console.error('Failed to delete media');
		} finally {
			isDeleteOpen = false;
		}
	}
</script>

<div class="flex h-screen flex-col xl:flex-row">
	<div class="flex items-center justify-center p-4 md:flex-1 md:p-0">
		<img
			class="aspect-2/3 max-h-[40vh] w-auto rounded-xl object-contain md:max-h-[60vh]"
			src={get_cover_url(data.cover_path) ?? ''}
			alt={`${data.name} cover image`}
		/>
	</div>

	<div class="flex flex-1 flex-col justify-center p-4 lg:pr-16">
		<div class="flex flex-col gap-2">
			<h1 class="font-serif text-xl md:text-3xl lg:text-4xl">
				{data.name}
			</h1>
			<h2 class="text-base md:text-xl">
				by <span class="font-medium">{data.metadata?.authors}</span>
			</h2>

			{#if data.metadata?.genres?.length}
				<div class="text-sm text-muted-foreground">
					{data.metadata.genres.join(', ')}
				</div>
			{/if}
		</div>

		<p class="mt-4 text-sm md:mt-8 md:text-base">{@html data.metadata?.description}</p>

		{#if data.progress}
			<div class="mt-2 flex items-center gap-2">
				{#if data.progress.completed}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Check size={18} />
						<span>Completed</span>
					</div>
				{:else}
					<Progress value={data.progress.percentage_completed} max={1} class="w-40" />
					<span class="text-sm text-muted-foreground">
						{Math.round(data.progress.percentage_completed * 100)}%
					</span>
				{/if}
			</div>
		{/if}

		<div class="mt-2 flex gap-2">
			<MetadataDialog bind:metadataSearch bind:isOpen={isMetadataOpen} name={data.name} />
			<Button size="icon" variant="outline"><Download /></Button>
			<Dialog
				title={`Delete ${data.name}`}
				description={`This action cannot be undone. ${data.name} will be permanently removed.`}
				bind:isOpen={isDeleteOpen}
				onClose={() => (isDeleteOpen = false)}
				onConfirm={handleDelete}
				cancelVariant="secondary"
				confirmVariant="destructive"
				confirmText="Delete"
				triggerVariant="destructive"
				triggerSize="icon"
			>
				{#snippet trigger()}
					<Trash2 />
				{/snippet}
			</Dialog>
			{#if metadataSearch}
				<Button onclick={saveMetadata}>Save</Button>
			{/if}
		</div>

		<div class="my-4 border-t-2"></div>

		<div class="grid grid-cols-2 gap-x-4 gap-y-2 sm:grid-cols-[auto_1fr_auto_1fr] sm:gap-x-8">
			{#each metadataArr as [label, value] (label)}
				<span class="text-sm font-medium sm:text-base">
					{labelize(label)}
				</span>
				<span class="text-sm wrap-break-word text-muted-foreground sm:text-base">
					{formatValue(label, value)}
				</span>
			{/each}
		</div>
	</div>
</div>

<script lang="ts">
	import type { components, operations } from '@shiori/api-client';

	import { goto, invalidate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Check from '@lucide/svelte/icons/check';
	import Download from '@lucide/svelte/icons/download';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import { createClient, get_cover_url } from '@shiori/api-client';
	import { Badge, Button, Progress } from '@shiori/components';
	import { format } from 'date-fns';

	import Dialog from '$lib/components/dialog.svelte';
	import MetadataDialog from '$lib/components/metadata/metadata-dialog.svelte';
	import ReadMore from '$lib/components/read-more.svelte';

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
		if (!value) return '—';

		if (key === 'published' && typeof value === 'string') {
			return format(value, 'PPP');
		}

		return Array.isArray(value) ? value.join(', ') : value;
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

<div class="mx-auto max-w-4xl px-4 py-8 md:px-8 md:py-12">
	<div class="flex flex-col items-center gap-8 md:gap-12 lg:flex-row lg:items-start">
		<div class="shrink-0">
			<div class="relative aspect-2/3 w-48 overflow-hidden rounded-lg shadow-lg md:w-64 lg:w-72">
				<img
					class="h-full w-full object-cover"
					src={get_cover_url(data.cover_path) ?? '/placeholder-book.jpg'}
					alt={`${data.name} cover`}
				/>
			</div>
		</div>

		<div class="flex flex-1 flex-col">
			<div class="mb-4">
				<h1 class="mb-2 font-serif text-3xl leading-tight font-bold md:text-4xl">
					{data.name}
				</h1>
				<div class="flex flex-wrap items-center gap-1">
					<span class="text-lg">by</span>
					<span class="font-medium">{data.metadata?.authors}</span>
				</div>

				{#if data.metadata?.genres?.length}
					<div class="mt-3 flex flex-wrap gap-2">
						{#each data.metadata.genres as genre}
							<Badge variant="secondary">{genre}</Badge>
						{/each}
					</div>
				{/if}
			</div>

			{#if data.metadata?.description}
				<ReadMore text={data.metadata?.description} class="text-base leading-relaxed md:text-lg" />
			{/if}

			{#if data.progress}
				<div class="mb-4 flex items-center justify-center gap-2 rounded-lg bg-accent/50 p-2">
					{#if data.progress.completed}
						<div class="flex items-center gap-2 text-emerald-600">
							<Check size={16} />
							<span class="text-sm font-medium">Completed</span>
						</div>
					{:else}
						<div class="flex w-full items-center gap-3">
							<Progress value={data.progress.percentage_completed} max={1} />
							<span class="text-xs font-medium tabular-nums">
								{Math.round(data.progress.percentage_completed * 100)}%
							</span>
						</div>
					{/if}
				</div>
			{/if}

			<div class="mb-4 flex flex-wrap items-center gap-2 border-b-2 border-border pb-4">
				<MetadataDialog bind:metadataSearch bind:isOpen={isMetadataOpen} name={data.name} />

				<Button size="icon" variant="ghost">
					<Download size={18} />
				</Button>

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
					<Button size="sm" onclick={saveMetadata} class="ml-auto">Save Changes</Button>
				{/if}
			</div>

			{#if metadataArr.length}
				<div class="grid grid-cols-2 gap-y-3">
					{#each metadataArr as [label, value] (label)}
						<div class="flex flex-col">
							<span class="text-xs font-semibold tracking-wide uppercase sm:text-sm">
								{labelize(label)}
							</span>
							<span class="mt-1 text-sm text-muted-foreground">
								{formatValue(label, value)}
							</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>

<script lang="ts">
	import {
		createClient,
		get_cover_url,
		type components,
		type operations
	} from '@shiori/api-client';
	import { Button } from '$lib/components/ui/button';
	import MetadataDialog from '$lib/components/metadata-dialog.svelte';

	import Download from '@lucide/svelte/icons/download';
	import Database from '@lucide/svelte/icons/database';
	import { invalidate } from '$app/navigation';

	type PatchMetadata = components['schemas']['PatchMetadata'];
	type MetadataSearch =
		operations['search_metadata']['responses']['200']['content']['application/json'];

	let { data } = $props();

	let client = createClient({ fetch });
	let isMetadataOpen = $state(false);

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

		if (key === 'published_at' && typeof value === 'string') {
			return new Date(value).toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});
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

	$inspect(metadataSearch);
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

		<div class="mt-4 flex gap-2">
			<Button onclick={() => (isMetadataOpen = true)} size="icon" variant="outline"
				><Database /></Button
			>
			<Button size="icon" variant="outline"><Download /></Button>
			{#if metadataSearch}
				<Button onclick={saveMetadata}>Save</Button>
			{/if}
		</div>

		<div class="my-4 border-t-2"></div>

		<div class="grid grid-cols-2 gap-x-4 gap-y-2 sm:grid-cols-[auto_1fr_auto_1fr] sm:gap-x-8">
			{#each metadataArr as [label, value]}
				{@render metadata(label, value)}
			{/each}
		</div>
	</div>
</div>

<MetadataDialog bind:metadataSearch bind:isOpen={isMetadataOpen} />

{#snippet metadata(key: string, value: string | string[] | null)}
	<span class="text-sm font-medium sm:text-base">
		{labelize(key)}
	</span>
	<span class="text-sm wrap-break-word text-muted-foreground sm:text-base">
		{formatValue(key, value)}
	</span>
{/snippet}

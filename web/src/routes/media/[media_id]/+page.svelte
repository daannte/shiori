<script lang="ts">
	import { get_cover_url } from '@shiori/api-client';
	import { Button } from '$lib/components/ui/button';

	import Database from '@lucide/svelte/icons/database';
	import Download from '@lucide/svelte/icons/download';

	let { data } = $props();

	let metadataArr = $derived.by(() =>
		data.metadata
			? Object.entries(data.metadata).filter(
					([key]) => !['genres', 'description', 'authors'].includes(key)
				)
			: []
	);
</script>

<div class="flex h-screen flex-col xl:flex-row">
	<div class="relative flex items-center justify-center p-4 md:flex-1 md:p-0">
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
			<Button size="icon" variant="outline"><Database /></Button>
			<Button size="icon" variant="outline"><Download /></Button>
		</div>

		<div class="my-4 border-t-2"></div>

		<div class="grid grid-cols-2 gap-x-4 gap-y-2 sm:grid-cols-[auto_1fr_auto_1fr] sm:gap-x-8">
			{#each metadataArr as [label, value]}
				{@render metadata(label, value ?? 'Unknown')}
			{/each}
		</div>
	</div>
</div>

{#snippet metadata(label: string, value: string | string[])}
	<span class="text-sm font-medium sm:text-base">
		{label.toLowerCase() === 'isbn'
			? 'ISBN'
			: label.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase())}
	</span>
	<span class="text-sm wrap-break-word text-muted-foreground sm:text-base">
		{#if label.toLowerCase() === 'published_at' && typeof value === 'string'}
			{new Date(value).toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			})}
		{:else}
			{value}
		{/if}
	</span>
{/snippet}

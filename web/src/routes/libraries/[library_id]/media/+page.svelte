<script lang="ts">
	import MediaCard from '$lib/components/media-card.svelte';
	import EmptyView from '$lib/components/empty-view.svelte';

	import BookText from '@lucide/svelte/icons/book-text';

	let { data } = $props();
</script>

<div class="flex p-6">
	{#if data.media.length > 0}
		<div
			class="grid w-full grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
		>
			{#each data.media as media (media.id)}
				<MediaCard {media} />
			{/each}
		</div>
	{:else}
		<EmptyView
			title="No media found"
			description="Upload your favorite media to get started"
			icon={BookText}
		>
			{#snippet content()}
				<p>TODO add button to upload media</p>
			{/snippet}
		</EmptyView>
	{/if}
</div>

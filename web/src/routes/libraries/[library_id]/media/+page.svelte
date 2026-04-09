<script lang="ts">
	import MediaCard from '$lib/components/media-card.svelte';
	import EmptyView from '$lib/components/empty-view.svelte';
	import { Button } from '$lib/components/ui/button';

	import BookText from '@lucide/svelte/icons/book-text';
	import UploadDialog from '$lib/components/upload/upload-dialog.svelte';
	import LibraryHeader from '$lib/components/library-header.svelte';

	let { data } = $props();

	let isUploadOpen = $state(false);
</script>

{#if data.media.length > 0}
	<div>
		<LibraryHeader bind:isOpen={isUploadOpen} />
		<div
			class="grid w-full grid-cols-2 gap-6 p-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
		>
			{#each data.media as media (media.id)}
				<MediaCard {media} />
			{/each}
		</div>
	</div>
{:else}
	<EmptyView
		title="No media found"
		description="Upload your favorite media to get started"
		icon={BookText}
	>
		{#snippet content()}
			<Button size="lg" onclick={() => (isUploadOpen = true)}>Upload Files</Button>
		{/snippet}
	</EmptyView>
{/if}

<UploadDialog id={data.libraryId} bind:isOpen={isUploadOpen} />

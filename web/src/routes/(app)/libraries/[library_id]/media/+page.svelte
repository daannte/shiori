<script lang="ts">
	import { invalidate } from '$app/navigation';
	import BookText from '@lucide/svelte/icons/book-text';
	import { createClient } from '@shiori/api-client';
	import { Button } from '@shiori/components';

	import Dialog from '$lib/components/dialog.svelte';
	import EmptyView from '$lib/components/empty-view.svelte';
	import LibraryHeader from '$lib/components/library-header.svelte';
	import MediaCard from '$lib/components/media-card.svelte';
	import Dropzone from '$lib/components/upload/dropzone.svelte';
	import FilesList from '$lib/components/upload/files-list.svelte';

	let client = createClient({ fetch });

	let { data } = $props();

	let isUploadOpen = $state(false);
	let isUploading = $state(false);
	let files = $state<File[]>([]);

	async function handleUpload() {
		if (!files.length) return;

		const formData = new FormData();
		files.forEach((f) => {
			formData.append('files', f);
		});

		try {
			let res = await client.POST('/api/v1/libraries/{id}/media', {
				params: { path: { id: data.libraryId } },
				// @ts-expect-error need to use `FormData` to send files
				body: formData
			});
			if (!res.data || res.error) throw new Error();
			invalidate('libraries:media');
		} catch (e) {
			console.error('Failed to upload files');
		} finally {
			isUploadOpen = false;
		}
	}
</script>

{#if data.media.length > 0}
	<div>
		<LibraryHeader bind:isOpen={isUploadOpen} />
		<div
			class="grid w-full grid-cols-2 gap-3 p-4 sm:grid-cols-3 sm:gap-6 lg:grid-cols-4 xl:grid-cols-6"
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

<Dialog
	title="Upload Files"
	bind:isOpen={isUploadOpen}
	isLoading={isUploading}
	onClose={() => {
		isUploadOpen = false;
		files = [];
	}}
	onConfirm={handleUpload}
	confirmText="Upload"
	cancelVariant="secondary"
>
	{#snippet children()}
		<Dropzone bind:files />
		<FilesList bind:files />
	{/snippet}
</Dialog>

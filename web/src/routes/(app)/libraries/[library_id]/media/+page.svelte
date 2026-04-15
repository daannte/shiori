<script lang="ts">
	import MediaCard from '$lib/components/media-card.svelte';
	import EmptyView from '$lib/components/empty-view.svelte';

	import BookText from '@lucide/svelte/icons/book-text';
	import LibraryHeader from '$lib/components/library-header.svelte';
	import Dialog from '$lib/components/dialog.svelte';
	import Dropzone from '$lib/components/upload/dropzone.svelte';
	import FilesList from '$lib/components/upload/files-list.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { createClient } from '@shiori/api-client';
	import { invalidate } from '$app/navigation';

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
	$inspect(isUploadOpen);
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

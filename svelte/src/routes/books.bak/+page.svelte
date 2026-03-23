<script lang="ts">
	import type { PageProps } from './$types';
	import { invalidate } from '$app/navigation';

	import * as Empty from '$lib/components/ui/empty';
	import { Button } from '$lib/components/ui/button';

	import Book from '@lucide/svelte/icons/book';

	let { data }: PageProps = $props();
	let fileInput: HTMLInputElement;
	let files = $state<FileList | null>();

	async function uploadBook() {
		if (!files || files.length == 0) {
			return;
		}

		const file = files[0];
		const formData = new FormData();
		formData.append('file', file);
		try {
			const res = await fetch(`/api/v1/books/`, {
				method: 'POST',
				body: formData
			});
			if (!res.ok) throw new Error('Failed to upload file');
			invalidate('app:books');
		} catch (err) {
			console.error(err);
		}
	}
</script>

<div class="p-6">
	{#if data.books && data.books.length > 0}
		<Button variant="outline" onclick={() => fileInput.click()}>Upload Book</Button>
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
			{#each data.books as book (book.id)}
				<a href={`/books/${book.id}`}>
					<img src={book.cover_image_url} alt={`${book.title} cover image`} class="h-96" />
					<p>{book.title}</p>
				</a>
			{/each}
		</div>
	{:else}
		<Empty.Root class="border border-dashed">
			<Empty.Header>
				<Empty.Media variant="icon">
					<Book />
				</Empty.Media>
				<Empty.Title>No Books</Empty.Title>
				<Empty.Description>Upload books to access them.</Empty.Description>
			</Empty.Header>
			<Empty.Content>
				<Button variant="outline" onclick={() => fileInput.click()}>Upload Book</Button>
			</Empty.Content>
		</Empty.Root>
	{/if}
</div>

<input
	aria-hidden="true"
	hidden
	type="file"
	accept=".epub"
	bind:files
	bind:this={fileInput}
	onchange={uploadBook}
/>

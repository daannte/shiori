<script lang="ts">
	import * as Empty from '$lib/components/ui/empty';
	import { Button } from '$lib/components/ui/button';
	import { Cloud } from '@lucide/svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<div class="p-6">
	{#if data.books && data.books.length > 0}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
			{#each data.books as book (book.id)}
				<a href={`/books/${book.id}`} class="bg-red-500">
					<img src={book.cover_image_url} alt={`${book.title} cover image`} class="h-96" />
					<p>{book.title}</p>
				</a>
			{/each}
		</div>
	{:else}
		<Empty.Root class="border border-dashed">
			<Empty.Header>
				<Empty.Media variant="icon">
					<Cloud />
				</Empty.Media>
				<Empty.Title>Cloud Storage Empty</Empty.Title>
				<Empty.Description>
					Upload files to your cloud storage to access them anywhere.
				</Empty.Description>
			</Empty.Header>
			<Empty.Content>
				<Button variant="outline" size="sm">Upload Files</Button>
			</Empty.Content>
		</Empty.Root>
	{/if}
</div>

<script lang="ts">
	import { createClient, type operations } from '@shiori/api-client';

	import * as Dialog from '../ui/dialog';
	import { Button } from '../ui/button';
	import { Label } from '../ui/label';
	import { Input } from '../ui/input';

	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import SearchCard from './search-card.svelte';

	type MetadataSearch =
		operations['get_book_metadata']['responses']['200']['content']['application/json'];

	type BooksMetadata =
		operations['search_books']['responses']['200']['content']['application/json'];

	interface Props {
		metadataSearch: MetadataSearch | undefined;
		isOpen: boolean;
		name: string;
	}

	let client = createClient({ fetch });

	let { metadataSearch = $bindable(), isOpen = $bindable(), name }: Props = $props();

	let title = $derived(name);
	let author = $state('');
	let books = $state.raw<BooksMetadata>([]);

	let loading = $state(false);

	async function getMetadata(id: number) {
		if (!id) return;

		loading = true;
		try {
			let res = await client.GET('/api/v1/metadata/book', {
				params: { query: { q: id.toString() } }
			});
			if (res.error || !res.data) throw new Error('Failed to get book metadata');
			metadataSearch = res.data;
			isOpen = false;
		} catch (e) {
			console.error('Failed book search: ', e);
		} finally {
			loading = false;
		}
	}

	async function search() {
		if (!(author.trim() || title.trim())) {
			return;
		}

		loading = true;
		try {
			let res = await client.GET('/api/v1/metadata/search', {
				params: { query: { title, author } }
			});
			if (res.error || !res.data) throw new Error('Failed to get books');
			books = res.data;
		} catch (e) {
			console.error('Failed books search: ', e);
		} finally {
			loading = false;
		}
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content class="sm:min-w-9/12">
		<Dialog.Header>
			<Dialog.Title class="text-xl font-semibold">Fetch Metadata</Dialog.Title>
			<Dialog.Description>Search by title and author to fetch metadata.</Dialog.Description>
		</Dialog.Header>

		<div class="mt-4 flex justify-center gap-4">
			<div>
				<Label for="author" class="sr-only">Author</Label>
				<Input id="author" bind:value={author} placeholder="Enter Author Name" />
			</div>

			<div>
				<Label for="title" class="sr-only">Book Title</Label>
				<Input id="title" bind:value={title} placeholder="Enter Book Title" />
			</div>
		</div>

		<div class="mt-6 grid gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
			{#each books as book}
				<SearchCard {book} onclick={getMetadata} />
			{/each}
		</div>

		<Button class="mt-4 w-full sm:w-auto" onclick={search} disabled={loading}>
			{#if loading}
				<LoaderCircle class="mr-2 inline-block animate-spin" />
				Searching...
			{:else}
				Search
			{/if}
		</Button>
	</Dialog.Content>
</Dialog.Root>

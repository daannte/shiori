<script lang="ts">
	import type { operations } from '@shiori/api-client';

	import Edit2 from '@lucide/svelte/icons/edit-2';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Search from '@lucide/svelte/icons/search';
	import { createClient } from '@shiori/api-client';
	import { Button, Input, Label } from '@shiori/components';

	import Dialog from '../dialog.svelte';
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

<Dialog
	bind:isOpen
	title="Fetch Metadata"
	onClose={() => (isOpen = false)}
	triggerVariant="outline"
	triggerSize="icon"
	class="flex max-h-[90vh] flex-col sm:min-w-11/12"
	hideFooter
>
	{#snippet trigger()}
		<Edit2 />
	{/snippet}

	{#snippet children()}
		<div class="mt-4 flex flex-col justify-center gap-2 sm:flex-row">
			<div>
				<Label for="author" class="sr-only">Author</Label>
				<Input
					class="text-sm md:text-base"
					id="author"
					bind:value={author}
					placeholder="Enter Author Name"
				/>
			</div>

			<div>
				<Label for="title" class="sr-only">Book Title</Label>
				<Input
					class="text-sm md:text-base"
					id="title"
					bind:value={title}
					placeholder="Enter Book Title"
				/>
			</div>
			<Button size="icon" class="w-full sm:w-8" onclick={search} disabled={loading}>
				{#if loading}
					<LoaderCircle class="animate-spin" />
				{:else}
					<Search />
				{/if}
			</Button>
		</div>
		{#if books.length}
			<div class="mt-6 grid gap-4 overflow-y-auto sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
				{#each books as book (book.provider_id)}
					<SearchCard {book} onclick={getMetadata} />
				{/each}
			</div>
		{/if}
	{/snippet}
</Dialog>

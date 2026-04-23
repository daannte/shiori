<script lang="ts">
	import type { operations } from '@shiori/api-client';

	import Edit2 from '@lucide/svelte/icons/edit-2';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Search from '@lucide/svelte/icons/search';
	import { createClient } from '@shiori/api-client';
	import { Button, Input, Label } from '@shiori/components';

	import Dialog from '../dialog.svelte';
	import SearchCard from './search-card.svelte';

	type BooksMetadata =
		operations['search_books']['responses']['200']['content']['application/json'];

	type Metadata = BooksMetadata[number];

	interface Props {
		metadataSearch: Metadata | undefined;
		isOpen: boolean;
		name: string;
		isbn: string | null | undefined;
	}

	let client = createClient({ fetch });

	let {
		metadataSearch = $bindable(),
		isOpen = $bindable(),
		name,
		isbn: searchIsbn
	}: Props = $props();

	let title = $derived(name);
	let author = $state('');
	let isbn = $derived(searchIsbn);
	let books = $state.raw<BooksMetadata>([]);

	let loading = $state(false);

	async function search() {
		if (!(author.trim() || title.trim() || isbn?.trim())) {
			return;
		}

		loading = true;
		try {
			let res = await client.GET('/api/v1/metadata/search', {
				params: { query: { q: isbn ? isbn : `${author} ${title}` } }
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
	description="If an ISBN is provided, it will be used instead of author and title."
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
		<div class="grid gap-3 sm:grid-cols-3 sm:items-end">
			<div class="space-y-1">
				<Label for="author" class="text-xs text-muted-foreground">Author</Label>
				<Input
					class="text-sm md:text-base"
					id="author"
					bind:value={author}
					placeholder="Asato Asato"
				/>
			</div>

			<div>
				<Label for="title" class="text-xs text-muted-foreground">Title</Label>
				<Input id="title" bind:value={title} placeholder="Eighty Six" />
			</div>
			<div>
				<Label for="isbn" class="text-xs text-muted-foreground">ISBN</Label>
				<Input id="isbn" bind:value={isbn} placeholder="9781975303129" />
			</div>
			<div class="flex justify-end sm:col-span-3">
				<Button class="w-full gap-2 sm:w-auto" onclick={search} disabled={loading}>
					{#if loading}
						<LoaderCircle class="animate-spin" />
						Searching...
					{:else}
						<Search />
						Search
					{/if}
				</Button>
			</div>
		</div>
		{#if books.length}
			<div class="mt-6 grid gap-4 overflow-y-auto sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
				{#each books as book (book.provider_id)}
					<SearchCard
						{book}
						onclick={() => {
							metadataSearch = book;
							isOpen = false;
						}}
					/>
				{/each}
			</div>
		{/if}
	{/snippet}
</Dialog>

<script lang="ts">
	import type { operations } from '@shiori/api-client';

	type MetadataSearch =
		operations['get_book_metadata']['responses']['200']['content']['application/json'];

	interface Props {
		book: MetadataSearch;
		onclick: (id: number) => void;
	}

	let { book, onclick }: Props = $props();
</script>

<button
	onclick={() => onclick(book.provider_id)}
	class="flex transform cursor-pointer flex-row gap-2 text-left transition duration-300 hover:scale-105 md:max-h-48"
>
	<img class="h-full rounded-lg object-cover" src={book.cover_url} alt="Book Cover" />

	<div class="flex flex-1 flex-col">
		<span class="font-semibold">{book.title}</span>
		<p class="text-sm">
			by <span class="font-medium">{book.authors}</span>
		</p>
		<p class="mt-2 line-clamp-5 text-sm text-muted-foreground">
			{@html book.description || 'No description available.'}
		</p>
	</div>
</button>

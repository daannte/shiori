<script lang="ts">
	import type { operations } from '@shiori/api-client';

	type Metadata =
		operations['search_books']['responses']['200']['content']['application/json'][number];

	interface Props {
		book: Metadata;
		onclick: () => void;
	}

	let { book, onclick }: Props = $props();
</script>

<button
	{onclick}
	class="flex flex-row gap-2 rounded-lg text-left transition-colors duration-200 hover:bg-muted md:max-h-48"
>
	<img
		class="aspect-2/3 h-24 rounded-lg object-cover md:h-full"
		src={book.cover_url}
		alt="Book Cover"
	/>

	<div class="flex flex-1 flex-col">
		<span class="text-sm font-semibold sm:text-base">{book.title}</span>
		<p class="text-sm">
			by <span class="font-medium">{book.authors}</span>
		</p>
		<p class="mt-2 line-clamp-5 text-xs text-muted-foreground sm:text-sm">
			{@html book.description || 'No description available.'}
		</p>
	</div>
</button>

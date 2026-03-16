<script lang="ts">
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Spinner } from '$lib/components/ui/spinner/index.js';
	import { IconCalendar, IconUser } from '@tabler/icons-svelte';
</script>

<div class="p-6">
	{#if data.books && data.books.length > 0}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
			{#each data.books as book (book.id)}
				<Card.Root class="rounded-lg border shadow transition hover:shadow-lg">
					<Card.Header class="flex items-center justify-between p-4">
						<Card.Title class="text-lg font-semibold">{book.title}</Card.Title>
					</Card.Header>
					<Card.Content class="space-y-2 p-4">
						<div class="flex items-center gap-2 text-sm text-gray-600">
							<IconUser size={16} />
							{book.author}
						</div>
						<div class="flex items-center gap-2 text-sm text-gray-600">
							{book.genre}
						</div>
						<div class="flex items-center gap-2 text-sm text-gray-600">
							<IconCalendar size={16} />
							{book.year}
						</div>
					</Card.Content>
				</Card.Root>
			{/each}
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center py-20 text-gray-500">
			<Spinner class="mb-4" />
			<p>No books found.</p>
		</div>
	{/if}
</div>

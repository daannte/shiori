<script lang="ts">
	import { goto } from '$app/navigation';

	import * as Empty from '$lib/components/ui/empty';
	import { Button } from '$lib/components/ui/button';

	import LibraryBig from '@lucide/svelte/icons/library-big';

	let { data } = $props();
</script>

<div class="min-h-screen p-6">
	{#if data.libraries}
		{#each data.libraries as library (library.id)}
			<div class="rounded-2xl bg-secondary p-4">
				<a href={`/libraries/${library.id}/media`}>
					<h3>{library.name}</h3>
					<p>Path: {library.path}</p>
				</a>
			</div>
		{/each}
	{:else}
		<Empty.Root class="min-h-screen">
			<Empty.Header>
				<Empty.Media variant="icon">
					<LibraryBig />
				</Empty.Media>
				<Empty.Title>No libraries found</Empty.Title>
				<Empty.Description>Click below to create your first library</Empty.Description>
			</Empty.Header>
			<Empty.Content>
				<Button variant="outline" onclick={() => goto('/libraries/new')}>Create Library</Button>
			</Empty.Content>
		</Empty.Root>
	{/if}
</div>

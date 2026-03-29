<script lang="ts">
	import { resolve } from '$app/paths';
	import { get_cover_url } from '@shiori/api-client';
	import * as Empty from '$lib/components/ui/empty';

	import BookText from '@lucide/svelte/icons/book-text';

	let { data } = $props();
</script>

<div class="flex p-6">
	{#if data.media.length > 0}
		<div
			class="grid w-full grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
		>
			{#each data.media as media (media.id)}
				<a
					href={resolve('/media/[media_id]', { media_id: media.id.toString() })}
					class="group overflow-hidden rounded-2xl bg-secondary p-2"
				>
					<div class="aspect-2/3 w-full overflow-hidden">
						<img
							class="h-full w-full rounded-xl object-cover"
							src={get_cover_url(media.cover_path) ?? ''}
							alt={`${media.name} cover image`}
						/>
					</div>

					<div class="p-2">
						<h3 class="text-lg font-medium">
							{media.name}
						</h3>
					</div>
				</a>
			{/each}
		</div>
	{:else}
		<Empty.Root class="min-h-screen">
			<Empty.Header>
				<Empty.Media variant="icon">
					<BookText />
				</Empty.Media>
				<Empty.Title>No media found</Empty.Title>
				<Empty.Description>Click below to upload new media</Empty.Description>
			</Empty.Header>
			<Empty.Content>
				<p>TODO add button to upload media</p>
			</Empty.Content>
		</Empty.Root>
	{/if}
</div>

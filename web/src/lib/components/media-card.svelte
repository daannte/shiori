<script lang="ts">
	import { resolve } from '$app/paths';
	import { get_cover_url } from '@shiori/api-client';

	let { media } = $props();
</script>

<a
	href={resolve('/(app)/media/[media_id]', { media_id: media.id.toString() })}
	class="group w-full"
>
	<div class="aspect-2/3 overflow-hidden rounded-2xl bg-secondary">
		<img
			class="h-full w-full object-cover transition duration-300 ease-out group-hover:scale-103"
			src={get_cover_url(media.cover_path) ?? ''}
			alt={`${media.name} cover image`}
		/>
	</div>

	<div class="mt-2 px-1">
		<h3 class="font-medium">
			{media.name}
		</h3>
	</div>
</a>

<script lang="ts">
	import { resolve } from '$app/paths';
	import Folders from '@lucide/svelte/icons/folders';
	import { Button } from '@shiori/components';

	import EmptyView from '$lib/components/empty-view.svelte';
</script>

<EmptyView
	title="No libraries yet"
	description="Create your first library to get started"
	icon={Folders}
>
	{#snippet content()}
		<Button size="lg" href={resolve('/libraries/create')}>Create Library</Button>
	{/snippet}
</EmptyView>

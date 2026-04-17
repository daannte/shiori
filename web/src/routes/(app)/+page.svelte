<script lang="ts">
	import { resolve } from '$app/paths';

	import EmptyView from '$lib/components/empty-view.svelte';
	import { Button } from '$lib/components/ui/button';
	import Folders from '@lucide/svelte/icons/folders';
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

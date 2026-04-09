<script lang="ts">
	import { page } from '$app/state';

	import { Button } from './ui/button';

	import Upload from '@lucide/svelte/icons/upload';

	interface Props {
		isOpen: boolean;
	}

	let { isOpen = $bindable() }: Props = $props();

	let id = $derived(page.params.library_id);
	$inspect(id);
</script>

<header class="sticky top-0 flex w-full border-b border-border p-2">
	<Button variant="ghost" onclick={() => (isOpen = true)}>
		<Upload />
	</Button>
</header>

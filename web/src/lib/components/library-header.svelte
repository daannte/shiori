<script lang="ts">
	import Upload from '@lucide/svelte/icons/upload';
	import { Button } from '@shiori/components';

	interface Props {
		isOpen: boolean;
	}

	let { isOpen = $bindable() }: Props = $props();
</script>

<header class="sticky top-0 flex w-full border-b border-border p-2">
	<Button variant="ghost" onclick={() => (isOpen = true)}>
		<Upload />
	</Button>
</header>

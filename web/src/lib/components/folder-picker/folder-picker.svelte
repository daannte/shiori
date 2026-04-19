<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import { Button } from '@shiori/components';

	import Dialog from '$lib/components/dialog.svelte';
	import FolderRow from './folder-row.svelte';

	interface Props {
		dirs: string[];
		parent: string | null;
		isOpen: boolean;
		path: string | null;
		selectedPath: string | null;
	}

	let {
		dirs,
		parent,
		isOpen = $bindable(),
		path = $bindable(),
		selectedPath = $bindable()
	}: Props = $props();
</script>

<Dialog
	title="Select Library Folder"
	description="Paths are relative to the application's base directory"
	bind:isOpen
	onClose={() => {
		isOpen = false;
		path = selectedPath;
	}}
	onConfirm={() => {
		isOpen = false;
		selectedPath = path;
	}}
	confirmText="Select"
	cancelVariant="secondary"
	triggerSize="icon"
	triggerVariant="ghost"
>
	<div class="flex items-center gap-2">
		{#if parent != null}
			<Button size="icon" onclick={() => (path = parent)}><ArrowLeft /></Button>
		{/if}
		<span class="w-full rounded-lg border-2 p-1 text-sm">
			{#if path}
				/{path}
			{:else}
				/
			{/if}
		</span>
	</div>
	<div class="mt-4 flex flex-col gap-2">
		{#each dirs as dir (dir)}
			<FolderRow {dir} onclick={(p: string) => (path = p)} />
		{/each}
	</div>
</Dialog>

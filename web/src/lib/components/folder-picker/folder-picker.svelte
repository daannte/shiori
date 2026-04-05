<script lang="ts">
	import * as Dialog from '../ui/dialog';
	import FolderRow from './folder-row.svelte';
	import { Button, buttonVariants } from '../ui/button';

	import ArrowLeft from '@lucide/svelte/icons/arrow-left';

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

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Select Library Folder</Dialog.Title>
			<Dialog.Description>Paths are relative to the application's base directory</Dialog.Description
			>
		</Dialog.Header>
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
			{#each dirs as dir}
				<FolderRow {dir} onclick={(p: string) => (path = p)} />
			{/each}
		</div>
		<Dialog.Footer>
			<Dialog.Close
				onclick={() => (path = selectedPath)}
				class={buttonVariants({ variant: 'secondary' })}>Cancel</Dialog.Close
			>
			<Button
				onclick={() => {
					selectedPath = path;
					isOpen = false;
				}}>Confirm</Button
			>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

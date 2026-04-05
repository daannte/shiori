<script lang="ts">
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import FileIcon from '@lucide/svelte/icons/file';
	import { Button } from '../ui/button';

	interface Props {
		files: File[];
	}

	let { files = $bindable() }: Props = $props();
</script>

<div>
	<div class="flex items-center justify-between">
		<span class="font-medium">Files</span>
		{#if files.length}
			<Button variant="outline" onclick={() => (files = [])}>Clear Files</Button>
		{/if}
	</div>
	{#if files.length}
		<div class="mt-2 flex flex-col gap-1">
			{#each files as file}
				<div class="group flex items-center justify-between rounded-xl bg-muted/80 p-2">
					<div class="flex items-center gap-2 overflow-hidden">
						<FileIcon size={20} class="shrink-0" />
						<span class="truncate font-medium">{file.name}</span>
					</div>
					<Button
						size="icon"
						variant="ghost"
						class="opacity-0 group-hover:opacity-100  hover:text-destructive"
						onclick={() => (files = files.filter((f) => f !== file))}
					>
						<Trash2 />
					</Button>
				</div>
			{/each}
		</div>
	{/if}
</div>

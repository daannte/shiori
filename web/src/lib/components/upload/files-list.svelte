<script lang="ts">
	import Trash2 from '@lucide/svelte/icons/trash-2';
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
			{#each files as file (file.name)}
				<div class="group flex items-center justify-between rounded-xl bg-muted/80 p-2">
					<span class="truncate font-medium">{file.name}</span>
					<Button
						size="icon"
						variant="ghost"
						class="opacity-100 lg:opacity-0 lg:group-hover:opacity-100"
						onclick={() => (files = files.filter((f) => f !== file))}
						aria-label="Delete File"
					>
						<Trash2 />
					</Button>
				</div>
			{/each}
		</div>
	{/if}
</div>

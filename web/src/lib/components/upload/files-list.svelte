<script lang="ts">
	import X from '@lucide/svelte/icons/x';
	import { Button } from '../ui/button';
	import * as Accordion from '$lib/components/ui/accordion/';

	interface Props {
		files: File[];
	}

	let { files = $bindable() }: Props = $props();
</script>

<div>
	<Accordion.Root type="single">
		<Accordion.Item value="item-1">
			<Accordion.Trigger disabled={!files.length}>Files ({files.length})</Accordion.Trigger>
			<Accordion.Content class="max-h-48 overflow-y-auto">
				{#if files.length}
					<div class="mt-2 flex flex-col gap-1">
						{#each files as file (file.name)}
							<div class="flex items-center justify-between rounded-xl bg-muted/80 p-2">
								<span class="truncate text-xs font-medium text-wrap sm:text-sm">{file.name}</span>
								<Button
									size="icon"
									variant="ghost"
									class=""
									onclick={() => (files = files.filter((f) => f !== file))}
									aria-label="Delete File"
								>
									<X />
								</Button>
							</div>
						{/each}
					</div>
				{/if}
			</Accordion.Content>
		</Accordion.Item>
	</Accordion.Root>
	<!-- <div class="flex items-center justify-between"> -->
	<!-- 	<span class="font-medium">Files</span> -->
	<!-- 	{#if files.length} -->
	<!-- 		<Button variant="outline" onclick={() => (files = [])}>Clear Files</Button> -->
	<!-- 	{/if} -->
	<!-- </div> -->
</div>

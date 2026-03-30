<script lang="ts">
	import * as Empty from '$lib/components/ui/empty';

	import { type LucideIcon } from '@lucide/svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		title: string;
		description: string;
		icon: LucideIcon;
		content: Snippet;
	}

	let { title, description, icon, content }: Props = $props();
</script>

<Empty.Root class="min-h-screen">
	<Empty.Header>
		<Empty.Media>
			{@const Icon = icon}
			<Icon class="text-muted-foreground" size={40} />
		</Empty.Media>
		<Empty.Title class="text-2xl md:text-3xl">{title}</Empty.Title>
		<Empty.Description class="sm:text-lg">{description}</Empty.Description>
	</Empty.Header>
	<Empty.Content>
		{@render content?.()}
	</Empty.Content>
</Empty.Root>

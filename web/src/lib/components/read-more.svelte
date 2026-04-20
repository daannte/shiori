<script lang="ts">
	import ChevronDown from '@lucide/svelte/icons/chevron-down';

	interface Props {
		text: string;
		class?: string;
	}

	let { text, class: className }: Props = $props();

	let isExpanded = $state(false);

	const MAX_LENGTH = 250;
	const truncated = $derived(text.length > MAX_LENGTH ? text.slice(0, MAX_LENGTH) : text);
	const needsTruncation = $derived(text.length > MAX_LENGTH);
</script>

<div class="relative text-left">
	<p class={className}>
		{@html isExpanded ? text : truncated}
	</p>

	{#if needsTruncation}
		<button
			class="mt-1 mb-4 inline-flex items-center gap-1 text-sm font-medium text-primary transition-colors hover:text-primary/80"
			onclick={() => (isExpanded = !isExpanded)}
			aria-expanded={isExpanded}
		>
			<span>{isExpanded ? 'Show less' : 'Read more'}</span>
			<ChevronDown
				size={16}
				class={`transition-transform duration-200 ${isExpanded ? 'rotate-180' : ''}`}
			/>
		</button>
	{/if}
</div>

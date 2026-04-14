<script lang="ts">
	import * as Dialog from './ui/dialog';
	import { Button, buttonVariants, type ButtonSize, type ButtonVariant } from './ui/button';
	import type { Snippet } from 'svelte';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';

	interface Props {
		isOpen: boolean;
		onConfirm?: () => void;
		onClose: () => void;
		title: string;
		description?: string;
		cancelText?: string;
		confirmText?: string;
		confirmVariant?: ButtonVariant;
		cancelVariant?: ButtonVariant;
		isLoading?: boolean;
		triggerVariant?: ButtonVariant;
		triggerSize?: ButtonSize;
		trigger?: Snippet | string;
		showFooter?: boolean;
		class?: string;
		children?: Snippet;
	}

	let {
		isOpen = $bindable(),
		onConfirm,
		onClose,
		title,
		description,
		cancelText,
		confirmText,
		confirmVariant,
		cancelVariant,
		isLoading,
		trigger,
		triggerSize,
		triggerVariant,
		showFooter = false,
		class: className,
		children
	}: Props = $props();

	function handleOnOpenChange(open: boolean) {
		if (!open && !isLoading) onClose();
	}
</script>

<Dialog.Root bind:open={isOpen} onOpenChange={handleOnOpenChange}>
	{#if trigger}
		<Dialog.Trigger
			{...typeof trigger !== 'string'
				? {
						type: 'button',
						class: buttonVariants({ variant: triggerVariant, size: triggerSize })
					}
				: {}}
		>
			{#if typeof trigger === 'string'}
				{trigger}
			{:else}
				{@render trigger()}
			{/if}
		</Dialog.Trigger>
	{/if}
	<Dialog.Content class={className}>
		<Dialog.Header>
			<Dialog.Title>{title}</Dialog.Title>
			<Dialog.Description>{description}</Dialog.Description>
		</Dialog.Header>
		{@render children?.()}
		{#if showFooter}
			<Dialog.Footer>
				<Button disabled={isLoading} onclick={onClose} variant={cancelVariant}
					>{cancelText || 'Cancel'}</Button
				>
				<Button disabled={isLoading} onclick={onConfirm} variant={confirmVariant}>
					{#if isLoading}
						<LoaderCircle class="animate-spin" />
					{:else}
						{confirmText || 'Confirm'}
					{/if}
				</Button>
			</Dialog.Footer>
		{/if}
	</Dialog.Content>
</Dialog.Root>

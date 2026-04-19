<script lang="ts">
	import { invalidate } from '$app/navigation';
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { createClient } from '@shiori/api-client';
	import { Button, DropdownMenu } from '@shiori/components';

	import Dialog from '../dialog.svelte';

	const client = createClient({ fetch });

	let { key_id }: { key_id: string } = $props();

	let isDeleteOpen = $state(false);
	let isDeleting = $state(false);

	async function handleDelete() {
		isDeleting = true;
		try {
			const res = await client.DELETE('/api/v1/tokens/{key_id}', { params: { path: { key_id } } });
			if (res.error) throw new Error();
			invalidate('tokens:load');
		} catch {
			console.error('Failed to delete token');
		} finally {
			isDeleting = false;
			isDeleteOpen = false;
		}
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon">
				<EllipsisIcon />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item variant="destructive" onclick={() => (isDeleteOpen = true)}
				>Delete</DropdownMenu.Item
			>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>

<Dialog
	title="Delete API Token"
	description="Deleting this API token will immediately revoke access for any services using it. This action cannot be undone."
	bind:isOpen={isDeleteOpen}
	onClose={() => (isDeleteOpen = false)}
	onConfirm={handleDelete}
	confirmText="Delete"
	cancelVariant="secondary"
	confirmVariant="destructive"
	isLoading={isDeleting}
/>

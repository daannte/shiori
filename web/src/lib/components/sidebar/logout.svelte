<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { createClient } from '@shiori/api-client';

	import * as Dialog from '../ui/dialog';
	import { Button, buttonVariants } from '../ui/button';
	import LogOut from '@lucide/svelte/icons/log-out';

	let client = createClient({ fetch });

	let isOpen = $state(false);

	async function handleLogout() {
		try {
			let res = await client.POST('/api/v1/auth/logout');
			if (res.error) throw new Error('Failed to logout');
			goto(resolve('/auth'), { replaceState: true });
		} catch (e) {
			console.error(e);
		} finally {
			isOpen = false;
		}
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Trigger type="button" class={buttonVariants({ size: 'icon', variant: 'ghost' })}
		><LogOut /></Dialog.Trigger
	>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Logout</Dialog.Title>
			<Dialog.Description>Do you really want to logout?</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button onclick={() => (isOpen = false)}>Cancel</Button>
			<Button onclick={handleLogout} variant="destructive">Logout</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

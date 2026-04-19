<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import LogOut from '@lucide/svelte/icons/log-out';
	import { createClient } from '@shiori/api-client';

	import Dialog from '$lib/components/dialog.svelte';

	let client = createClient({ fetch });

	let isOpen = $state(false);
	let isLoading = $state(false);

	async function handleLogout() {
		isLoading = true;

		try {
			let res = await client.POST('/api/v1/auth/logout');
			if (res.error) throw new Error('Failed to logout');
			goto(resolve('/auth'), { replaceState: true });
		} catch (e) {
			console.error(e);
		} finally {
			isLoading = false;
			isOpen = false;
		}
	}
</script>

<Dialog
	title="Logout"
	description="Do you really want to logout?"
	bind:isOpen
	{isLoading}
	onClose={() => (isOpen = false)}
	onConfirm={handleLogout}
	confirmText="Logout"
	cancelVariant="secondary"
	triggerSize="icon"
	triggerVariant="ghost"
>
	{#snippet trigger()}
		<LogOut />
	{/snippet}
</Dialog>

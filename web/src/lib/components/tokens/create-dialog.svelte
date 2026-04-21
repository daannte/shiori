<script lang="ts">
	import type { DateValue } from '@internationalized/date';

	import { invalidate } from '$app/navigation';
	import { getLocalTimeZone } from '@internationalized/date';
	import Plus from '@lucide/svelte/icons/plus';
	import { createClient } from '@shiori/api-client';
	import { Input, Label } from '@shiori/components';

	import Dialog from '../dialog.svelte';
	import DatePicker from './date-picker.svelte';

	let client = createClient({ fetch });

	let isOpen = $state(false);
	let name = $state('');
	let date = $state<DateValue>();
	let createdToken = $state<string | null>(null);
	let isLoading = $state(false);

	function handleClose() {
		name = '';
		date = undefined;
		createdToken = null;
		isLoading = false;
		isOpen = false;
	}

	async function handleToken() {
		if (!name) return;

		isLoading = true;

		try {
			const res = await client.POST('/api/v1/tokens', {
				body: {
					expires_at: date?.toDate(getLocalTimeZone()).toISOString() ?? null,
					name
				}
			});

			if (!res.data || res.error) throw new Error();

			createdToken = res.data.plaintoken;
			invalidate('tokens:load');
		} catch {
			console.error('Failed to generate token');
		} finally {
			isLoading = false;
		}
	}
</script>

<Dialog
	bind:isOpen
	title={createdToken ? 'Token Created' : 'Generate Token'}
	description={createdToken
		? 'Make sure to copy your token now. You won’t be able to see it again.'
		: 'Provide details to generate a new token.'}
	onClose={handleClose}
	onConfirm={createdToken ? () => navigator.clipboard.writeText(createdToken!) : handleToken}
	confirmText={createdToken ? 'Copy' : 'Generate'}
	cancelText={createdToken ? 'Done' : 'Cancel'}
	cancelVariant={'secondary'}
	triggerSize="sm"
	{isLoading}
>
	{#snippet trigger()}
		<Plus /> New Token
	{/snippet}

	{#snippet children()}
		{#if createdToken}
			<div class="flex flex-col gap-4">
				<Input
					readonly
					value={createdToken}
					class="font-mono"
					onfocus={(e) => e.currentTarget.select()}
				/>
			</div>
		{:else}
			<div class="flex flex-col gap-4">
				<div class="grid gap-2">
					<Label for="name">Name</Label>
					<Input id="name" name="name" placeholder="Koreader Sync" bind:value={name} />
				</div>
				<div class="grid gap-2">
					<Label>
						Expiration Date
						<span class="text-muted-foreground">(optional)</span>
					</Label>

					<DatePicker bind:date />

					<p class="text-xs text-muted-foreground">Leave empty to never expire</p>
				</div>
			</div>
		{/if}
	{/snippet}
</Dialog>

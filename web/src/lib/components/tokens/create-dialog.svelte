<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { createClient } from '@shiori/api-client';

	import * as Dialog from '../ui/dialog';
	import { Label } from '../ui/label';
	import { Input } from '../ui/input';
	import { Button, buttonVariants } from '../ui/button';
	import DatePicker from './date-picker.svelte';
	import { type DateValue, getLocalTimeZone } from '@internationalized/date';

	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Plus from '@lucide/svelte/icons/plus';

	let client = createClient({ fetch });

	let isOpen = $state(false);
	let name = $state('');
	let date = $state<DateValue>();
	let createdToken = $state<string | null>(null);
	let isLoading = $state(false);

	function reset() {
		name = '';
		date = undefined;
		createdToken = null;
		isLoading = false;
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

<Dialog.Root
	bind:open={isOpen}
	onOpenChange={(open) => {
		if (!open) reset();
	}}
>
	<Dialog.Trigger type="button" class={buttonVariants({ variant: 'default' })}>
		<Plus /> Generate New Token
	</Dialog.Trigger>
	<Dialog.Content showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>
				{createdToken ? 'Token Created' : 'Generate Token'}
			</Dialog.Title>
			<Dialog.Description>
				{#if createdToken}
					Make sure to copy your token now. You won’t be able to see it again.
				{:else}
					Provide details to generate a new token.
				{/if}
			</Dialog.Description>
		</Dialog.Header>

		{#if createdToken}
			<div class="flex flex-col gap-4">
				<Input
					readonly
					value={createdToken}
					class="font-mono"
					onfocus={(e) => e.currentTarget.select()}
				/>

				<div class="flex gap-2">
					<Button class="flex-1" onclick={() => navigator.clipboard.writeText(createdToken!)}>
						Copy
					</Button>

					<Button
						variant="secondary"
						onclick={() => {
							reset();
							isOpen = false;
						}}
					>
						Done
					</Button>
				</div>
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

			<Dialog.Footer>
				<Dialog.Close class={buttonVariants({ variant: 'secondary' })}>Cancel</Dialog.Close>

				<Button onclick={handleToken} disabled={!name || isLoading}>
					{#if isLoading}
						<LoaderCircle class="animate-spin" />
					{:else}
						Generate
					{/if}
				</Button>
			</Dialog.Footer>
		{/if}
	</Dialog.Content>
</Dialog.Root>

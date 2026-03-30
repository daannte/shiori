<script lang="ts">
	import { createClient, type operations } from '@shiori/api-client';

	import * as Dialog from './ui/dialog';
	import { Button } from './ui/button';
	import { Label } from './ui/label';
	import { Input } from './ui/input';

	import LoaderCircle from '@lucide/svelte/icons/loader-circle';

	type MetadataSearch =
		operations['search_metadata']['responses']['200']['content']['application/json'];

	interface Props {
		metadataSearch: MetadataSearch | undefined;
		isOpen: boolean;
	}

	let client = createClient({ fetch });

	let { metadataSearch = $bindable(), isOpen = $bindable() }: Props = $props();

	let externalId = $state('');
	let loading = $state(false);
	let error = $state('');

	async function search() {
		error = '';
		if (!externalId) {
			error = 'Provide an ID';
			return;
		}

		loading = true;
		try {
			let res = await client.GET('/api/v1/metadata/search', {
				params: { query: { q: externalId } }
			});
			if (res.error || !res.data) throw new Error('Failed to get metadata');
			metadataSearch = res.data;
		} catch (e) {
			console.error('Failed metadata: ', e);
			error = 'Failed to fetch metadata';
		} finally {
			loading = false;
			isOpen = false;
		}
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Fetch Metadata</Dialog.Title>
			<Dialog.Description>Enter the external ID (Goodreads) to fetch metadata.</Dialog.Description>
		</Dialog.Header>
		<div class="mt-4 flex flex-col gap-4">
			<div class="grid flex-1 gap-2">
				<Label for="externalId" class="sr-only">External ID</Label>
				<Input id="externalId" bind:value={externalId} placeholder="Enter Goodreads ID" />
			</div>

			<Button class="mt-2" onclick={search} disabled={loading}>
				{#if loading}
					<LoaderCircle class="animate-spin" />
				{:else}
					Search
				{/if}
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>

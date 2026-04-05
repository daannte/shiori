<script lang="ts">
	import { createClient } from '@shiori/api-client';

	import * as Dialog from '../ui/dialog';
	import { Button, buttonVariants } from '../ui/button';

	import FilesList from './files-list.svelte';
	import Dropzone from './dropzone.svelte';

	interface Props {
		isOpen: boolean;
	}

	let client = createClient({ fetch });

	let { isOpen = $bindable() }: Props = $props();

	let files = $state<File[]>([]);

	async function uploadFiles() {
		if (!files.length) return;
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Upload Files</Dialog.Title>
		</Dialog.Header>
		<Dropzone bind:files />
		<FilesList bind:files />
		<Dialog.Footer>
			<Dialog.Close onclick={() => (files = [])} class={buttonVariants({ variant: 'secondary' })}
				>Cancel</Dialog.Close
			>
			<Button onclick={uploadFiles}>Upload</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

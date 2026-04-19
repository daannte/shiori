<script lang="ts">
	import CloudUpload from '@lucide/svelte/icons/cloud-upload';
	import { Button } from '@shiori/components';

	interface Props {
		files: File[];
	}

	let { files = $bindable() }: Props = $props();
	let inputEl: HTMLInputElement;
	let isDragActive = $state(false);

	function addFiles(newFiles: FileList) {
		const fileArray = Array.from(newFiles);
		const existing = new Set(files.map((f) => f.name));
		const filtered = fileArray.filter((f) => !existing.has(f.name));
		files = [...files, ...filtered];
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragActive = false;

		let newFiles = e.dataTransfer?.files;
		if (newFiles) addFiles(newFiles);
	}
</script>

<div
	class={`flex items-center justify-center rounded-lg border-2 border-dotted border-border p-4 ${isDragActive ? 'border-primary bg-muted' : ''}`}
	role="button"
	tabindex="0"
	ondrop={handleDrop}
	ondragleave={() => (isDragActive = false)}
	ondragover={(e) => {
		e.preventDefault();
		isDragActive = true;
	}}
>
	<div class="flex flex-col items-center text-sm">
		<CloudUpload class="text-muted-foreground" size={48} />

		<div class="mt-2 text-center">
			<p>Drag & drop files here or</p>

			<p class="-mt-2">
				<Button variant="link" class="p-0 text-primary" onclick={() => inputEl.click()}
					>choose files
				</Button>
				to upload
			</p>
		</div>
	</div>
</div>

<input
	id="fileUpload"
	type="file"
	bind:this={inputEl}
	multiple
	hidden
	accept="application/epub+zip"
	onchange={(e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target.files) addFiles(target.files);
	}}
/>

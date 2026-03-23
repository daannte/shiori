<script lang="ts">
	import type { PageProps } from './$types';

	import * as Dialog from '$lib/components/ui/dialog';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	import Download from '@lucide/svelte/icons/download';
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import Database from '@lucide/svelte/icons/database';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import { goto } from '$app/navigation';

	let { data }: PageProps = $props();

	let metadata = $state.raw<Object>();

	let externalId = $state('');
	let loading = $state(false);
	let error = $state('');

	async function searchMetadata() {
		error = '';
		if (!externalId) {
			error = 'Please enter an external ID.';
			return;
		}

		loading = true;
		try {
			const res = await fetch(
				'/api/v1/metadata/search?' +
					new URLSearchParams({
						external_id: externalId,
						source: 'goodreads'
					})
			);

			if (!res.ok) throw new Error('Error when looking up metadata');

			metadata = await res.json();
		} catch (err) {
			console.error('search failed: ', err);
			error = 'Failed to fetch metadata.';
		} finally {
			loading = false;
		}
	}

	// TODO: Handle this func better
	async function saveMetadata() {
		if (!metadata) return;

		const res = await fetch(`/api/v1/books/${data.id}/enrich/`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				...metadata,
				external_source: 'goodreads',
				external_id: externalId
			})
		});

		if (!res.ok) throw new Error('Failed to save metadata');
		const updatedBook = await res.json();
		data = updatedBook;
	}

	async function deleteBook() {
		const res = await fetch(`/api/v1/books/${data.id}/`, {
			method: 'DELETE'
		});

		if (!res.ok) throw new Error('Failed to delete book');
		goto('/books');
	}
</script>

<div class="mx-auto h-screen max-w-full p-8">
	<div class="mx-auto flex max-w-5xl">
		<img src={data.cover_image_url} alt={`${data.title} cover image`} class="h-96" />

		<div class="mt-8 ml-16 flex flex-col gap-4">
			<h1 class="text-4xl font-semibold">{data.title}</h1>
			<p class="flex flex-wrap items-center gap-2 text-lg text-gray-600">
				{#each data.authors as author, i}
					<span>{author.name}{i < data.authors.length - 1 ? ',' : ''}</span>
				{/each}
			</p>

			<div class="mt-auto flex justify-end gap-4 p-4">
				<Button href="/books" aria-label="Back" variant="outline" size="icon" class="mr-auto">
					<ArrowLeft />
				</Button>

				<Dialog.Root>
					<Dialog.Trigger class={buttonVariants({ variant: 'outline' })}>
						<Database />
					</Dialog.Trigger>

					<Dialog.Content class="sm:max-w-md">
						<Dialog.Header>
							<Dialog.Title>Fetch Metadata</Dialog.Title>
							<Dialog.Description>
								Enter the external ID (Goodreads) to fetch metadata.
							</Dialog.Description>
						</Dialog.Header>

						<div class="mt-4 flex flex-col gap-4">
							<div class="grid flex-1 gap-2">
								<Label for="externalId" class="sr-only">External ID</Label>
								<Input id="externalId" bind:value={externalId} placeholder="Enter Goodreads ID" />
							</div>

							{#if error}
								<p class="text-sm text-red-500">{error}</p>
							{/if}

							<Button class="mt-2" onclick={searchMetadata} disabled={loading}>
								{#if loading}Loading...{/if}
								{#if !loading}Search{/if}
							</Button>

							{#if metadata}
								{JSON.stringify(metadata)}
								<Button aria-label="Save" onclick={saveMetadata}>Save</Button>
							{/if}
						</div>
					</Dialog.Content>
				</Dialog.Root>

				<Button aria-label="Download" variant="outline" size="icon">
					<Download />
				</Button>

				<Button aria-label="Delete" variant="ghost" size="icon" onclick={deleteBook}>
					<Trash2 />
				</Button>
			</div>

			<div class="border border-border"></div>
		</div>
	</div>

	<div class="mx-auto mt-16 flex max-w-7xl gap-8 p-8 px-16">
		<div class="flex-1">
			<h3 class="mb-4 text-2xl font-semibold">Description</h3>
			<p class="leading-relaxed text-gray-700">{@html data.description}</p>
		</div>

		<div class="ml-16 flex-1 flex-col gap-6">
			<div>
				<h3 class="mb-2 text-2xl font-semibold">Language</h3>
				<p class="leading-relaxed text-gray-700">{data.language}</p>
			</div>

			<div>
				<h3 class="mb-2 text-2xl font-semibold">Pages</h3>
				<p class="leading-relaxed text-gray-700">{data.pages}</p>
			</div>

			<div>
				<h3 class="mb-2 text-2xl font-semibold">Published</h3>
				<p class="leading-relaxed text-gray-700">
					{#if data.publication_time}
						{new Date(data.publication_time).toLocaleDateString('en-US', {
							year: 'numeric',
							month: 'long',
							day: 'numeric'
						})}
					{/if}
				</p>
			</div>
		</div>
	</div>
</div>

<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '@shiori/components';

	const DEFAULT_MESSAGE = 'Page not found';
</script>

<div class="flex min-h-screen flex-col items-center justify-center gap-4">
	<h1 class="text-2xl">
		{page.status === 404 ? DEFAULT_MESSAGE : (page.error?.message ?? DEFAULT_MESSAGE)}
	</h1>

	{#if page.error?.tryAgain}
		<Button onclick={() => location.reload()}>Reload</Button>
	{/if}
</div>

<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/app-sidebar.svelte';

	let { data, children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Sidebar.Provider>
	<div class="flex h-full flex-1">
		<AppSidebar {data} />
		<main class="flex flex-1 flex-col overflow-x-hidden overflow-y-auto">
			<div class="flex flex-1 flex-col">
				{@render children()}
			</div>
		</main>
	</div>
</Sidebar.Provider>

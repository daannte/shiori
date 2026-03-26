<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/app-sidebar.svelte';

	let { data, children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Sidebar.Provider>
	<AppSidebar {data} />
	<main>
		<Sidebar.Trigger />
		{@render children()}
	</main>
</Sidebar.Provider>

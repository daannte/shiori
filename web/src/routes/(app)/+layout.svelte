<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/sidebar/sidebar.svelte';

	let { data, children } = $props();
</script>

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

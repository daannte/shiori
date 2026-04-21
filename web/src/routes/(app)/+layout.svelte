<script lang="ts">
	import { Sidebar } from '@shiori/components';

	import AppSidebar from '$lib/components/sidebar/sidebar.svelte';

	let { data, children } = $props();
</script>

<Sidebar.Provider>
	<div class="flex h-full flex-1 overflow-hidden">
		<AppSidebar {data} />
		<main class="flex flex-1 flex-col overflow-y-auto">
			<div class="flex flex-1 flex-col">
				{@render children()}
			</div>
		</main>
	</div>
</Sidebar.Provider>

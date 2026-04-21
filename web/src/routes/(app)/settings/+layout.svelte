<script lang="ts">
	import { Separator, Sidebar } from '@shiori/components';

	import SettingsSidebar from '$lib/components/settings/sidebar.svelte';

	let { children } = $props();
</script>

<Sidebar.Provider>
	<div class="flex w-full max-w-screen flex-1 flex-col p-4">
		<div class="flex flex-row items-center">
			<Sidebar.Trigger class="inline-flex md:hidden" />
			<h1 class="text-lg font-semibold md:text-2xl">Settings</h1>
		</div>
		<Separator class="my-2 md:my-4" />
		<div class="flex flex-row">
			<SettingsSidebar />
			<div class="grow overflow-x-auto p-0 md:p-4">
				{@render children()}
			</div>
		</div>
	</div>
</Sidebar.Provider>

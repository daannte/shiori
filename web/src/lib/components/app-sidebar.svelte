<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar';

	let { data } = $props();
</script>

<Sidebar.Root>
	<Sidebar.Header />
	<Sidebar.Content>
		<Sidebar.Group />
		<Sidebar.Group />
	</Sidebar.Content>
	<Sidebar.Footer />
</Sidebar.Root>

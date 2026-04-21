<script lang="ts">
	import CreateDialog from '$lib/components/tokens/create-dialog.svelte';
	import DataTable from '$lib/components/tokens/data-table.svelte';

	let { data } = $props();
</script>

<div class="flex items-center justify-between">
	<h1 class="text-lg font-medium md:text-xl">Tokens List</h1>
	<CreateDialog />
</div>
<DataTable data={data.tokens} />

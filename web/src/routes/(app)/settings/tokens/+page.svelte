<script lang="ts">
	import { Label } from '@shiori/components';

	import CreateDialog from '$lib/components/tokens/create-dialog.svelte';
	import DataTable from '$lib/components/tokens/data-table.svelte';

	let { data } = $props();
</script>

<div class="flex items-center justify-between">
	<Label class="text-base md:text-lg">Tokens List</Label>
	<CreateDialog />
</div>
<DataTable data={data.tokens} />

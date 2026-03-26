<script lang="ts">
	import { goto } from '$app/navigation';

	import * as Empty from '$lib/components/ui/empty';
	import { Button } from '$lib/components/ui/button';

	import Folders from '@lucide/svelte/icons/folders';
</script>

<Empty.Root class="min-h-screen">
	<Empty.Header>
		<Empty.Media>
			<Folders size={40} />
		</Empty.Media>
		<Empty.Title class="text-2xl md:text-3xl">No libraries yet</Empty.Title>
		<Empty.Description class="sm:text-lg">
			Create your first library to get started
		</Empty.Description>
	</Empty.Header>
	<Empty.Content>
		<Button size="lg" onclick={() => goto('/libraries/new')}>Create Library</Button>
	</Empty.Content>
</Empty.Root>

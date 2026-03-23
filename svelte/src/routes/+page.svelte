<script lang="ts">
	import { goto } from '$app/navigation';

	import * as Empty from '$lib/components/ui/empty';
	import { Button } from '$lib/components/ui/button';

	import LibraryBig from '@lucide/svelte/icons/library-big';
</script>

<Empty.Root class="min-h-screen">
	<Empty.Header>
		<Empty.Media variant="icon">
			<LibraryBig />
		</Empty.Media>
		<Empty.Title>No libraries found</Empty.Title>
		<Empty.Description>Click below to create your first library</Empty.Description>
	</Empty.Header>
	<Empty.Content>
		<Button variant="outline" onclick={() => goto('/libraries/new')}>Create Library</Button>
	</Empty.Content>
</Empty.Root>

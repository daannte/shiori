<script>
	import { Label, Select } from '@shiori/components';
	import { resetMode, setMode, userPrefersMode } from 'mode-watcher';

	let mode = $derived(
		userPrefersMode.current.charAt(0).toUpperCase() + userPrefersMode.current.slice(1)
	);
</script>

<div class="flex flex-row items-center justify-between">
	<Label class="text-base md:text-lg">Appearance</Label>
	<Select.Root type="single" value={userPrefersMode.current}>
		<Select.Trigger size="sm" class="w-24">
			{mode}
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="light" onclick={() => setMode('light')}>Light</Select.Item>
			<Select.Item value="dark" onclick={() => setMode('dark')}>Dark</Select.Item>
			<Select.Item value="system" onclick={() => resetMode()}>System</Select.Item>
		</Select.Content>
	</Select.Root>
</div>

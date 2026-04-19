<script lang="ts">
	import type { DateValue } from '@internationalized/date';

	import { getLocalTimeZone } from '@internationalized/date';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import { Button, Calendar, Popover } from '@shiori/components';
	import { formatDate } from 'date-fns';

	import { cn } from '$lib/utils.js';

	let { date = $bindable() }: { date: DateValue | undefined } = $props();
</script>

<Popover.Root>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button
				variant="outline"
				class={cn(
					'w-[280px] justify-start text-start font-normal',
					!date && 'text-muted-foreground'
				)}
				{...props}
			>
				<CalendarIcon class="me-2 size-4" />
				{date ? formatDate(date.toDate(getLocalTimeZone()), 'PPP') : 'Select a date'}
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-auto p-0">
		<Calendar bind:value={date} type="single" initialFocus captionLayout="dropdown" />
	</Popover.Content>
</Popover.Root>

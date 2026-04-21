<script lang="ts" module>
	import House from '@lucide/svelte/icons/house';
	import Unplug from '@lucide/svelte/icons/unplug';

	const data = {
		navMain: [
			{
				title: 'Account',
				url: '#',
				items: [
					{
						title: 'General',
						path: '/settings',
						url: resolve('/(app)/settings'),
						icon: House
					},
					{
						title: 'Tokens',
						path: '/settings/tokens',
						url: resolve('/(app)/settings/tokens'),
						icon: Unplug
					}
				]
			}
		]
	};
</script>

<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { IsMobile, Sidebar, useSidebar } from '@shiori/components';

	const isMobile = new IsMobile();
	const sidebar = useSidebar();

	function isActive(path: string): boolean {
		return page.url.pathname === path;
	}
</script>

<Sidebar.Root class="w-40 md:bg-transparent" collapsible={isMobile.current ? 'offcanvas' : 'none'}>
	<Sidebar.Content>
		{#each data.navMain as group (group.title)}
			<Sidebar.Group>
				<Sidebar.GroupLabel>{group.title}</Sidebar.GroupLabel>
				<Sidebar.GroupContent>
					<Sidebar.Menu>
						{#each group.items as item (item.title)}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton onclick={() => sidebar.toggle()} isActive={isActive(item.path)}>
									{#snippet child({ props })}
										<a href={item.url} {...props}>
											<item.icon />
											{item.title}
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/each}
					</Sidebar.Menu>
				</Sidebar.GroupContent>
			</Sidebar.Group>
		{/each}
	</Sidebar.Content>
</Sidebar.Root>

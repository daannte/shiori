<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import Logout from './logout.svelte';

	import ChevronUp from '@lucide/svelte/icons/chevron-up';
	import House from '@lucide/svelte/icons/house';
	import BookText from '@lucide/svelte/icons/book-text';
	import Settings2 from '@lucide/svelte/icons/settings-2';
	import { Button } from '../ui/button';

	let { data } = $props();

	function isActive(path: string): boolean {
		return page.url.pathname === path;
	}
</script>

<Sidebar.Root>
	<Sidebar.Header />
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.Menu class="gap-2">
				<Sidebar.MenuItem>
					<Sidebar.MenuButton isActive={isActive('/')}>
						{#snippet child({ props })}
							<a href={resolve('/')} {...props}>
								<House />
								<span>Home</span>
							</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton isActive={isActive('/media')}>
						{#snippet child({ props })}
							<a href={resolve('/media')} {...props}>
								<BookText />
								<span>Explore</span>
							</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Group>
		<Sidebar.Group>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Collapsible.Root open class="group/collapsible">
						<Sidebar.MenuItem>
							<Collapsible.Trigger class="font-medium">
								{#snippet child({ props })}
									<Sidebar.MenuButton {...props}>
										Libraries
										<ChevronUp
											class="ms-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-180"
										/>
									</Sidebar.MenuButton>
								{/snippet}
							</Collapsible.Trigger>
							<Collapsible.Content>
								<Sidebar.MenuSub>
									{#each data.libraries as library (library.id)}
										<Sidebar.MenuSubItem>
											<Sidebar.MenuSubButton isActive={isActive(`/libraries/${library.id}/media`)}>
												{#snippet child({ props })}
													<a
														href={resolve('/(app)/libraries/[library_id]/media', {
															library_id: library.id.toString()
														})}
														{...props}
													>
														{library.name}
													</a>
												{/snippet}
											</Sidebar.MenuSubButton>
										</Sidebar.MenuSubItem>
									{/each}
								</Sidebar.MenuSub>
							</Collapsible.Content>
						</Sidebar.MenuItem>
					</Collapsible.Root>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer class="flex flex-row justify-between">
		<Button size="icon" variant="ghost" href={resolve('/(app)/settings')}>
			<Settings2 />
		</Button>
		<Logout />
	</Sidebar.Footer>
</Sidebar.Root>

<script lang="ts">
	import { page } from '$app/state';

	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as Collapsible from '$lib/components/ui/collapsible';

	import ChevronUp from '@lucide/svelte/icons/chevron-up';

	let { data } = $props();

	function isActive(library_id: number): boolean {
		return page.url.pathname === `/libraries/${library_id}/media`;
	}
</script>

<Sidebar.Root>
	<Sidebar.Header />
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Libraries</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Collapsible.Root open class="group/collapsible">
						<Sidebar.MenuItem>
							<Collapsible.Trigger>
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
											<Sidebar.MenuSubButton isActive={isActive(library.id)}>
												{#snippet child({ props })}
													<a href={`/libraries/${library.id}/media`} {...props}>
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
	<Sidebar.Footer />
</Sidebar.Root>

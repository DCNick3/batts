<script>
	import '../app.postcss'
	import '../style.css'
	import logo from '$lib/assets/batts_logo_512.png'
	import { Sidebar, SidebarWrapper, SidebarGroup, SidebarItem } from 'flowbite-svelte'
</script>

<div class="page">
	<Sidebar asideClass="w-64 bg-slate-50">
		<SidebarWrapper divClass="space-y-auto rounded dark:bg-gray-700 p-4">
			<SidebarGroup>
				<SidebarItem href="/" spanClass="mx-auto text-lg">
					<svelte:fragment slot="icon">
						<img src={logo} alt="A green bat"/>
					</svelte:fragment>
				</SidebarItem>
			</SidebarGroup>
		</SidebarWrapper>
	</Sidebar>

	<div class="flex flex-col w-full p-10 gap-6">
		<slot />
	</div>
</div>

<style>
	.page {
		display: flex;
		flex: 1 1 100%;
		height: 100vh;
	}
</style>

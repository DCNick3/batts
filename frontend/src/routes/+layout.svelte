<script lang="ts">
	import '../app.postcss'
	import Logo from '$lib/components/Logo.svelte'
	import { setContext } from 'svelte'
	import { writable } from 'svelte/store'
	import { Button } from 'flowbite-svelte'
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Dropdown, DropdownItem } from 'flowbite-svelte'
	import NavLink from '$lib/components/NavLink.svelte'

	import type { LayoutData } from './$types'
	import type { UserView } from 'backend'
	import { goto } from '$app/navigation';
	export let data: LayoutData

	const user = writable<null | UserView>()
	$: user.set(data.user)

	setContext('user', user)

	const goToLogin = () => {
		goto('/login')
	}
</script>

<div class="flex max-md:flex-col grow shrink basis-full h-screen">
	<Navbar class="md:hidden p-4 bg-slate-50">
		<NavBrand href="/">
			<Logo class="w-8 h-8" />
		</NavBrand>
		{#if $user == null}
			<Button
				on:click={goToLogin}
				class="text-sm p-1"
			>
				Login
			</Button>
		{:else}
			<NavLink class="text-sm" href="/me">{$user.name}</NavLink>
			<NavHamburger class="m-0" />
			<Dropdown>
				<DropdownItem href="/">Open a Ticket</DropdownItem>
				<DropdownItem href="/assigned">Assigned Tickets</DropdownItem>
			</Dropdown>
			{/if}
	</Navbar>
	<aside class="max-md:hidden flex flex-col items-center w-64 bg-slate-50 gap-6 p-4">
		<a href="/" class="block w-fit">
			<Logo />
		</a>
		{#if $user === null}
			<Button
				on:click={goToLogin}
				class="text-md"
			>
				Login
			</Button>
		{:else}
			<NavLink href="/me">{$user.name}</NavLink>
			<NavLink href="/assigned">Assigned Tickets</NavLink>
		{/if}
	</aside>

	<div class="flex flex-col w-full p-10 gap-6">
		<slot />
	</div>
</div>


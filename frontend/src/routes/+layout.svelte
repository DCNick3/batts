<script lang="ts">
	import '../app.postcss'
	import Logo from '$lib/components/Logo.svelte'
	import { setContext } from 'svelte'
	import { writable } from 'svelte/store'
	import { Button } from 'flowbite-svelte'
	import { Navbar, NavBrand } from 'flowbite-svelte'
	// import { } from 'flowbite-svelte'
	import NavHamburger from '$lib/components/NavHamburger.svelte'
	import NavLink from '$lib/components/NavLink.svelte'
	import { SidePanel } from '$lib/components/SidePanel'

	import type { LayoutData } from './$types'
	import type { UserView } from 'backend'
	import { goto } from '$app/navigation';
	export let data: LayoutData

	let isHidden = true

	const user = writable<null | UserView>()
	$: user.set(data.user)

	setContext('user', user)

	const goToLogin = () => {
		goto('/login')
	}
</script>

<Navbar
	class="px-4 sm:px-10 border-b relative h-14"
	fluid
>
	<div class="flex gap-3">
		<NavHamburger
			class="sm:hidden"
			on:click={() => { isHidden = !isHidden; console.log("ABOBA") }}
		/>
		<NavBrand
			class="font-semibold"
			href="/"
		>
			<Logo class="w-8 h-8 mr-2" />
			Batts
		</NavBrand>
	</div>
	{#if $user === null}
		<Button
			on:click={goToLogin}
			class="text-sm p-1"
		>
			Login
		</Button>
	{:else}
		<NavLink class="text-sm" href="/me">{$user.name}</NavLink>
	{/if}
</Navbar>

<div class="flex flex-col sm:flex-row grow shrink basis-full h-screen">
	{#if $user !== null}
			<SidePanel bind:hidden={isHidden} />
	{/if}
	<!-- <aside class="max-md:hidden flex flex-col items-center w-64 bg-slate-50 gap-6 p-4">
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
	</aside> -->

	<div class="flex flex-col w-full p-8 gap-6">
		<slot />
	</div>
</div>

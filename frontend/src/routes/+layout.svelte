<script lang="ts">
	import '../app.postcss'
	import Logo from '$lib/components/Logo.svelte'
	import { getContext, setContext } from 'svelte'
	import { writable, type Writable } from 'svelte/store'
	import { Button } from 'flowbite-svelte'
	import { Navbar, NavBrand } from 'flowbite-svelte'
  import Icon from '@iconify/svelte'
	import NavHamburger from '$lib/components/NavHamburger.svelte'
	import NavLink from '$lib/components/NavLink.svelte'
	import { SidePanel } from '$lib/components/SidePanel'

	import type { LayoutData } from './$types'
	import type { UserView, GroupView } from 'backend'
	import { goto } from '$app/navigation'
	import { pushApiError, pushError } from '$lib'

	export let data: LayoutData

	const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
	if (data.error) {
		if (data.error.type === 'Api') {
			pushApiError(errorContext, data.error.error)
		} else {
			pushError(errorContext, data.error.error)
		}
	}

	let isHidden = true

	const user = writable<null | UserView>()
	$: user.set(data.user)
	const userGroups = writable<GroupView[]>()
	$: userGroups.set(data.userGroups)

	setContext('user', user)
	setContext('userGroups', userGroups)

	const error = writable<{ title: string, message: string }[]>([])
	setContext('error', error)

	const removeError = (index: number) => {
		error.update(errors => {
			return errors.toSpliced(index, 1)
		})
	}

	const goToLogin = () => {
		goto('/login')
	}
</script>

<Navbar
	class="px-4 sm:px-10 border-b relative sm:fixed h-14 z-50"
	fluid
>
	<div class="flex gap-3">
		{#if $user !== null}
			<NavHamburger
				class="sm:hidden"
				on:click={() => { isHidden = !isHidden }}
			/>
		{/if}
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

<div class="flex flex-col sm:flex-row grow shrink basis-full">
	{#if $user !== null}
			<SidePanel bind:hidden={isHidden} />
	{/if}

	<div class="flex flex-col w-full p-4 sm:p-8 gap-6 sm:ml-64 sm:mt-14">
		<slot />
	</div>
</div>

{#if $error && $error.length > 0}
	<div class="absolute right-0 bottom-0 mr-2 mb-2 sm:m-12">
		{#each $error as error, index}
			<div class="p-2 border rounded-md bg-red-200 mt-2 flex gap-1">
				<div>
					<div>{error.title}</div>
					<div>{error.message}</div>		
				</div>
				<button class="h-fit" on:click={() => removeError(index)}>
					<Icon icon="fa:remove" style="width: 10px; height: 10px;" />
				</button>
			</div>
		{/each}
	</div>
{/if}

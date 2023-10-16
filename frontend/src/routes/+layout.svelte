<script lang="ts">
	import '../app.postcss'
	import Logo from '$lib/components/Logo.svelte'
	import { setContext } from 'svelte'
	import { writable } from 'svelte/store'

	import type { LayoutData } from './$types'
	import type { UserView } from 'backend'
	export let data: LayoutData

	const user = writable<null | UserView>()
	$: user.set(data.user)

	setContext('user', user)
</script>

<div class="flex grow shrink basis-full h-screen">
	<aside class="flex flex-col items-center w-64 bg-slate-50 p-4">
		<a href="/" class="block w-fit">
			<Logo />
		</a>
	</aside>

	<div class="flex flex-col w-full p-10 gap-6">
		<slot />
	</div>
</div>


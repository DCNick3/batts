<script lang="ts">
	import { TicketList } from '$lib/components/TicketList'
	import type { Writable } from 'svelte/store'
	import type { PageData } from './$types'
	import { getContext } from 'svelte'
	import { pushApiError, pushError } from '$lib'

	export let data: PageData

	const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
	if (data.error) {
		if (data.error.type === 'Api') {
			pushApiError(errorContext, data.error.error)
		} else {
			pushError(errorContext, data.error.error)
		}
	}
</script>

<svelte:head>
	<title>Assigned tickets</title>
</svelte:head>

{#if data.tickets.length === 0}
	<h1 class="mx-auto mt-10 text-xl font-semibold">There are no tickets yet</h1>
{:else}	
	<TicketList
		tickets={data.tickets}
		users={data.userMap}
		groups={data.groupMap}
		displaySubmitter
	/>
{/if}

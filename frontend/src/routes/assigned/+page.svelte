<script lang="ts">
	import { TicketList } from '$lib/components/TicketList'
	import type { PageData } from './$types'

	export let data: PageData

</script>

{#if data.tickets.length === 0}
	<h1 class="mx-auto mt-10 text-xl font-semibold">There are no tickets yet</h1>
{:else}	
	<TicketList
		tickets={data.tickets}
		users={data.userMap}
		groups={data.groupMap}
	/>
{/if}

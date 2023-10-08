<script lang="ts">
	import TicketList from '$lib/components/TicketList.svelte'
	import type { PageData } from './$types'

	export let data: PageData

	$: tickets = data.requests.map(r => ({
		receiver: r.sender,
		...r
	}))
</script>

<TicketList tickets={tickets}/>

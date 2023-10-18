<script lang="ts">
	import TicketList from '$lib/components/TicketList.svelte'
	import type { PageData } from './$types'

	export let data: PageData

</script>

<TicketList tickets={data.tickets}/>

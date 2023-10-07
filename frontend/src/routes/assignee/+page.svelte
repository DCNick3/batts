<script>
  import {
    Table,
		TableHead,
		TableHeadCell,
		TableBody,
		TableBodyCell,
    Timeline,
    TimelineItem
  } from 'flowbite-svelte';
	import TicketList from '$lib/components/TicketList.svelte'

	export let data

	$: ticks = data.requests.map(r => ({
		receiver: r.sender,
		...r
	}))
</script>

<TicketList tickets={ticks}/>

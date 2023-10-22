<script lang="ts">
	import type { TicketListingViewExpandedItem } from 'backend'
  import StatusBadge from '$lib/components/StatusBadge.svelte';

  export let ticket: TicketListingViewExpandedItem
</script>

<div class="grid grid-cols-[1fr_2fr] border rounded-sm">
  <div class="whitespace-nowrap text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5">Submitted To</div>
  <div class="px-3 py-2 text-sm text-slate-500">{ticket.destination} <StatusBadge status={ticket.status}/></div>

  <div class="text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5">Topic</div>
  <div class="text-base px-3 py-2">{ticket.title}</div>
  
</div>

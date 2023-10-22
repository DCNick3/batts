<script lang="ts">
	import StatusBadge from '$lib/components/StatusBadge.svelte'
	import type { TicketListingViewExpandedItem } from 'backend'
	import TicketListItem from './TicketListItem.svelte'

	export let tickets: TicketListingViewExpandedItem[]

</script>

<div class="flex flex-col gap-2">
	{#each tickets as ticket}
		<TicketListItem {ticket} />
	{/each}
</div>
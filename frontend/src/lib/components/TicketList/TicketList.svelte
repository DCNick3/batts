<script lang="ts">
	import StatusBadge from '$lib/components/StatusBadge.svelte'
	import type {GroupProfileView, TicketListingViewExpandedItem, UserProfileView} from 'backend'
	import TicketListItem from './TicketListItem.svelte'

	export let tickets: TicketListingViewExpandedItem[]
	export let users: Record<string, UserProfileView>
	export let groups: Record<string, GroupProfileView>

	export let displaySubmitter: boolean = false

</script>

<div class="flex flex-col gap-2 sm:gap-0">
	<div class="max-sm:hidden flex whitespace-nowrap text-xs uppercase text-gray-700 bg-gray-50 font-semibold rounded-t-md border-x border-t">
		<div class="w-32 px-6 py-3">Status</div>
		<div class="w-48 py-3">Submitted To</div>
		{#if displaySubmitter}
			<div class="w-48 py-3">Requested By</div>
		{/if}
		<div class="w-fit py-3">Topic</div>
	</div>
	{#each tickets as ticket}
		<TicketListItem {ticket} {users} {groups} {displaySubmitter} />
	{/each}
</div>
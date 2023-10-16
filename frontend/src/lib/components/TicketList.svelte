<script lang="ts">
  import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow
	} from 'flowbite-svelte'
	import StatusBadge from '$lib/components/StatusBadge.svelte'
	import type { TicketListingViewExpandedItem } from 'backend'

	export let tickets: TicketListingViewExpandedItem[]

</script>

<Table hoverable color="default" class="border rounded-md border-separate border-spacing-0">
	<colgroup>
		<col class="w-16"/>
		<col class="w-48"/>
		<col class="w-fit"/>
	</colgroup>
	<TableBody tableBodyClass="rounded-sm">
		{#each tickets as ticket}
		<!-- TODO: Three hardcoded false's come from the `ticket.up` property -->
			<TableBodyRow class={"first:rounded-t-sm last:rounded-b-sm" + (true ? "" : " bg-gray-50")}>
				<TableBodyCell
					class="px-2 text-sm rounded-l-md text-center"
				>
					<StatusBadge status={ticket.status} />
				</TableBodyCell>
				<TableBodyCell
					class="text-sm text-slate-500"
				>
					<a href="/">
						{ticket.destination}
					</a>
				</TableBodyCell>
				<TableBodyCell
					class={`text-base font-${false ? 'semibold' : 'medium'} rounded-r-md`}
				>
					<a href={`/tickets/${ticket.id}`} class="flex">
						{#if false}
							<div class="w-2 h-2 rounded bg-primary-600 mr-1 self-center" />
						{/if}
						{ticket.title}
					</a>
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

<script lang="ts">
  import {
		Table,
		TableHead,
		TableHeadCell,
		TableBody,
		TableBodyCell,
		TableBodyRow
	} from 'flowbite-svelte'
	import StatusBadge from '$lib/components/StatusBadge.svelte'
	import type { TicketListingViewExpandedItem } from 'backend'

	export let tickets: TicketListingViewExpandedItem[]

</script>

<Table
	hoverable
	color="default"
	class="border rounded-md border-separate border-spacing-0 flex flex-row flex-no-wrap sm:inline-table"
	divClass=""
>
	<colgroup class="hidden sm:table-column-group">
		<col class="w-16"/>
		<col class="w-48"/>
		<col class="w-fit"/>
	</colgroup>
	<TableHead defaultRow={false} class="bg-white">
		{#each tickets as _}
			<tr class="sm:hidden sm:first:table-row flex flex-col flex-nowrap last:mb-0 mb-2 sm:mb-0 bg-gray-50">
				<TableHeadCell class="p-2.5 sm:px-6">Status</TableHeadCell>
				<TableHeadCell class="p-2.5 sm:px-6">Submitted To</TableHeadCell>
				<TableHeadCell class="p-3 sm:px-6">Topic</TableHeadCell>
			</tr>
		{/each}
	</TableHead>
	<TableBody tableBodyClass="rounded-sm flex-1 sm:flex-none">
		{#each tickets as ticket}
		<!-- TODO: Three hardcoded false's come from the `ticket.up` property -->
			<TableBodyRow class={"flex sm:table-row flex-col flex-nowrap first:rounded-t-sm last:rounded-b-sm last:mb-0 mb-2 sm:mb-0 border-b-0" + (true ? "" : " bg-gray-50")}>
				<TableBodyCell
					class="py-2 sm:py-4 sm:px-2 text-sm rounded-l-md sm:text-center"
				>
					<StatusBadge status={ticket.status} />
				</TableBodyCell>
				<TableBodyCell
					class="py-2 sm:py-4 text-sm text-slate-500"
				>
					<a href="/">
						{ticket.destination}
					</a>
				</TableBodyCell>
				<TableBodyCell
					class={`py-2 sm:py-4 text-base font-${false ? 'semibold' : 'medium'} rounded-r-md`}
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

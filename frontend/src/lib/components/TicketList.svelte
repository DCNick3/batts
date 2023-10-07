<script lang="ts">
  import {
		Badge,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow
	} from 'flowbite-svelte'

  type Ticket = { status: string, receiver: string, topic: string, up: boolean }
	export let tickets: Ticket[]

	function status2color (status: string) {
		if (status === "Pending") return "yellow"
		if (status === "Fixed") return "green"
		if (status === "In process") return "blue"
		return "primary"
	}
</script>

<Table hoverable color="default" class="border rounded-md border-separate border-spacing-0">
	<colgroup>
		<col class="w-16"/>
		<col class="w-48"/>
		<col class="w-fit"/>
	</colgroup>
	<TableBody tableBodyClass="rounded-sm">
		{#each tickets as ticket}
			<TableBodyRow class={"first:rounded-t-sm last:rounded-b-sm" + (ticket.up ? " bg-gray-50" : "")}>
				<TableBodyCell
					class="px-2 text-sm rounded-l-md text-center"
				>
					<Badge rounded color={status2color(ticket.status)}>
						{ticket.status}
					</Badge>
				</TableBodyCell>
				<TableBodyCell
					class="text-sm text-slate-500"
				>
					<a href="/">
						{ticket.receiver}
					</a>
				</TableBodyCell>
				<TableBodyCell
					class="text-base font-semibold rounded-r-md"
				>
					<a href="/ticket">
						{ticket.topic}
					</a>
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

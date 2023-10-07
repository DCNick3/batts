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
  export let data
</script>

<Table>
	<TableHead>
		<TableHeadCell>Topic</TableHeadCell>
		<TableHeadCell>Status</TableHeadCell>
	</TableHead>
		{#each data.requests as ticket}
			<TableBody>
				<TableBodyCell>{ticket.topic}</TableBodyCell>
				<TableBodyCell>{ticket.status}</TableBodyCell>
			</TableBody>
		{/each}
</Table>
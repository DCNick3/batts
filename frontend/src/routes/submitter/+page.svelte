<script>
	import {
		Badge,
		Button,
		Input,
		Label,
		Table,
		TableHead,
		TableHeadCell,
		TableBody,
		TableBodyCell,
		Textarea,
		Timeline,
		TimelineItem, 
		TableBodyRow
	} from 'flowbite-svelte'
	// @ts-ignore
	import AutoComplete from "simple-svelte-autocomplete"

	import { twMerge } from 'tailwind-merge'
	let defaultClass = 'text-gray-900 bg-gray-50 border border-gray-300 rounded-lg focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500';
	$: autocompleteClass = twMerge('block w-full', defaultClass, 'text-sm p-2.5', $$props.class);

	// @ts-ignore
	export let data;

	// @ts-ignore
	function status2color (status) {
		if (status === "Pending") return "yellow"
		if (status === "Fixed") return "green"
		if (status === "In process") return "blue"
		return "primary"
	}
</script>

<form
	method="POST"
	action="?/submit"
	class="flex flex-col gap-4"
>
	<Label>
		Submit To:
		<AutoComplete
			className="block w-full my-1"
			class={autocompleteClass}
			items={data.receivers}
			labelFieldName="name"
			valueFieldName="id"
			placeholder="Choose option..."
			hideArrow
			required
		/>
	</Label>
	<Label>
		Topic:
		<Input class="mt-1" name="topic" required />
	</Label>
	<Label>
		Description:
		<Textarea
			class="mt-1"
			name="description"
			rows=8
			required
		/>
	</Label>
</form>

<Button>Submit</Button>

<h1 class="mx-auto mt-10 text-xl font-semibold">Submitted requests</h1>

<Table hoverable color="default" class="border rounded-md border-separate border-spacing-0">
	<colgroup>
		<col class="w-16"/>
		<col class="w-48"/>
		<col class="w-fit"/>
	</colgroup>
	<TableBody tableBodyClass="rounded-sm">
		{#each data.requests as ticket}
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
					<a href="/">
						{ticket.topic}
					</a>
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

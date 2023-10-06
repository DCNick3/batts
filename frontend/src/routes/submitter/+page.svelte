<script>
	import { Label, Input, Textarea, Select, Button, Timeline, TimelineItem } from 'flowbite-svelte'
	import AutoComplete from "simple-svelte-autocomplete"

	import { twMerge } from 'tailwind-merge'
	export let defaultClass = 'text-gray-900 bg-gray-50 border border-gray-300 rounded-lg focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500';
	$: autocompleteClass = twMerge('block w-full', defaultClass, 'text-sm p-2.5', $$props.class);

	export let data;
</script>

<form
	method="POST"
	action="?/create"
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
	<!-- <Label>
		Submit To:
		<Select
			class="mt-1"
			items={data.receivers.map(r => ({ value : r.id, name : r.name }))}
			required
		/>
	</Label> -->
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

<Timeline order="vertical" class="mx-auto mt-6">
  {#each data.requests as ticket}
    <TimelineItem title={ticket.topic}>
      <p><strong>To:</strong> {ticket.receiver}</p>
      <p><strong>Status:</strong> {ticket.status}</p>
    </TimelineItem>
  {/each}
</Timeline>
<!-- <ul>
	{#each data.requests as request}
		<li><strong>To:</strong> {request.receiver}, <strong>Topic:</strong> {request.topic}, <strong>Status:</strong> {request.status}</li>
	{/each}
</ul> -->


<!-- <style>

	ul {
		margin: 0;
		padding: 0;
		list-style: none;
	}

	li {
		margin: 10px 0;
	}

</style> -->
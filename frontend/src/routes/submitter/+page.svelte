<script>
	import { Label, Input, Textarea, Select, Button } from 'flowbite-svelte'
	export let data;
</script>

<form
	method="POST"
	action="?/create"
	class="flex flex-col gap-4"
>
	<Label>
		Submit To:
		<Select
			class="mt-1"
			items={data.receivers.map(r => ({ value : r.id, name : r.name }))}
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

<h1>Submitted requests</h1>

<ul>
	{#each data.requests as request}
		<li><strong>To:</strong> {request.receiver}, <strong>Topic:</strong> {request.topic}, <strong>Status:</strong> {request.status}</li>
	{/each}
</ul>


<style>

	h1 {
		margin: 40px auto 0;
	}

	ul {
		margin: 0;
		padding: 0;
		list-style: none;
	}

	li {
		margin: 10px 0;
	}

</style>
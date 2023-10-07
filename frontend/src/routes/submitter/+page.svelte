<script>
	import {
		Button,
		Input,
		Label,
		Textarea,
	} from 'flowbite-svelte'
	import TicketList from '$lib/components/TicketList.svelte'
	// @ts-ignore
	import AutoComplete from "simple-svelte-autocomplete"

	import { twMerge } from 'tailwind-merge'
	let defaultClass = 'text-gray-900 bg-gray-50 border border-gray-300 rounded-lg focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500';
	$: autocompleteClass = twMerge('block w-full', defaultClass, 'text-sm p-2.5', $$props.class);

	// @ts-ignore
	export let data;

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

<TicketList tickets={data.requests} />

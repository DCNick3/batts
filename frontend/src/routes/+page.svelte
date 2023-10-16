<script lang="ts">
	import {
		Button,
		Input,
		Label,
		Textarea,
	} from 'flowbite-svelte'
	import AutoComplete from '$lib/components/AutoComplete.svelte'
	import TicketList from '$lib/components/TicketList.svelte'
  import { goto } from '$app/navigation'
	import type { PageData } from './$types'
	import { Api, generateId, type TicketDestination } from 'backend'

	let destination: {name: string, id: string}

	const submit = async (event: SubmitEvent) => {
		if (!event.target) return
		const formData = new FormData(event.target as HTMLFormElement)
		// TODO: use ts properly
		const topic = formData.get("topic") as string
		const description = formData.get("description") as string
		// const destination = formData.get("destination") as TicketDestination

		const api = new Api(fetch)
		const newId = generateId()
		const result = await api.createTicket(newId, { title: topic, body: description, destination: destination.id as TicketDestination})
		// TODO: handle error
		if (result.status === 'Success') {
			goto(`/tickets/${newId}`)
		} else {
			console.log(result.payload)
		}
  }

	export let data: PageData;

</script>

<form
	on:submit|preventDefault={submit}
	class="flex flex-col gap-4"
>
	<Label>
		Submit To:
		<AutoComplete
			bind:selectedItem={destination}
			class="w-full"
			items={data.receivers}
			labelFieldName="name"
			valueFieldName="id"
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
	<Button type="submit">Submit</Button>
</form>

{#if data.requests.length === 0}
	<h1 class="mx-auto mt-10 text-xl font-semibold">You have no submitted requests</h1>
{:else}
	<h1 class="mx-auto mt-10 text-xl font-semibold">Submitted requests</h1>
	<TicketList tickets={data.requests} />
{/if}


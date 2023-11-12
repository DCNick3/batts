<script lang="ts">
	import {
		Button,
		Input,
		Label,
		Textarea,
	} from 'flowbite-svelte'
	import AutoComplete from '$lib/components/AutoComplete.svelte'
  import { goto } from '$app/navigation'
	import type { PageData } from './$types'
	import { Api, generateId } from 'backend'
  import { getContext } from 'svelte'
	import type { UserView } from 'backend'
	import { page } from '$app/stores'
	import { TicketList } from '$lib/components/TicketList'

	const url = $page.url

  const user = getContext<SvelteStore<null | UserView>>('user')
	let destination: {name: string, id: string}

	const qName = url.searchParams.get('gname')
	const qId = url.searchParams.get('gid')
	if (qName && qId) {
		destination = {
			name: qName,
			id: qId
		}
	}

	const submit = async (event: SubmitEvent) => {
		if (!event.target) return
		const formData = new FormData(event.target as HTMLFormElement)
		// TODO: use ts properly
		const topic = formData.get("topic") as string
		const description = formData.get("description") as string

		const api = new Api(fetch)
		const newId = generateId()
		const result = await api.createTicket(newId, { title: topic, body: description, destination: { type: 'Group', id: destination.id }})
		// TODO: handle error
		if (result.status === 'Success') {
			goto(`/tickets/${newId}`)
		} else {
			console.error(result.payload)
		}
  }

	export let data: PageData

</script>

<svelte:head>
	<title>Main</title>
</svelte:head>

{#if $user !== null}
	<form
		on:submit|preventDefault={submit}
		class="flex flex-col gap-4"
	>
		<Label>
			Submit To:
			<AutoComplete
				bind:selectedItem={destination}
				class="my-1"
				inputClass="w-full"
				items={data.receivers}
				labelFieldName="name"
				valueFieldName="id"
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
		<Button type="submit">Submit</Button>
	</form>

	{#if data.ownedTickets.length === 0}
		<h1 class="mx-auto mt-10 text-xl font-semibold">You have no submitted requests</h1>
	{:else}
		<h1 class="mx-auto mt-10 text-xl font-semibold">Submitted requests</h1>
		<TicketList
			tickets={data.ownedTickets}
			users={data.userMap}
			groups={data.groupMap}
		/>
		<div class="w-1 h-1 mt-10" />
	{/if}
{/if}

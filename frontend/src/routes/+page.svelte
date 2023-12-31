<script lang="ts">
	import {
		Button,
		Input,
		Label,
		Textarea,
	} from 'flowbite-svelte'
  import { goto } from '$app/navigation'
	import type { PageData } from './$types'
	import { Api, generateId } from 'backend'
  import { getContext } from 'svelte'
	import type { GroupView, UserView } from 'backend'
	import { page } from '$app/stores'
	import { TicketList } from '$lib/components/TicketList'
	import { UserAndGroupSearch, pushApiError, pushError } from '$lib'
	import type { Writable } from 'svelte/store'

	export let data: PageData

	const url = $page.url

  const user = getContext<SvelteStore<null | UserView>>('user')
	let destination: { type: 'User', view: UserView } | { type: 'Group', view: GroupView }
	let topic: string
	let description: string

  const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
	if (data.error) {
		if (data.error.type === 'Api') {
			pushApiError(errorContext, data.error.error)
		} else {
			pushError(errorContext, data.error.error)
		}
	}

	const qName = url.searchParams.get('gname')
	const qId = url.searchParams.get('gid')
	if (qName && qId) {
		destination = {
			type: 'Group',
			view: {
				id: qId,
				title: qName,
				members: [] // not required for purposes of destination
			}
		}
	}

	const submit = async () => {

		const api = new Api(fetch)
		const newId = generateId()
		const result = await api.createTicket(newId, { title: topic, body: description, destination: { type: destination.type, id: destination.view.id }})
		if (result.status === 'Success') {
			goto(`/tickets/${newId}`)
		} else {
			console.error(result.payload)
			pushApiError(errorContext, result.payload)
		}
  }

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
			<UserAndGroupSearch
				class="my-1"
				inputClass="w-full"
				placeholder="Dorm,  IT,  319"
				bind:destination={destination}
				required
			/>
		</Label>
		<Label>
			Topic:
			<Input bind:value={topic} class="mt-1" name="topic" required />
		</Label>
		<Label>
			Description:
			<Textarea
				class="mt-1"
				bind:value={description}
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

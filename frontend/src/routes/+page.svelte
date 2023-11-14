<script lang="ts">
	import {
		Button,
		Input,
		Label,
		TabItem,
		Textarea,
	} from 'flowbite-svelte'
  import { goto } from '$app/navigation'
	import type { PageData } from './$types'
	import { Api, generateId } from 'backend'
  import { getContext } from 'svelte'
	import type { ApiResult, GroupView, SearchResults, UserView } from 'backend'
	import { page } from '$app/stores'
	import { TicketList } from '$lib/components/TicketList'
	import { AutoComplete } from '$lib'

	const url = $page.url

  const user = getContext<SvelteStore<null | UserView>>('user')
	let destination: { type: 'User', view: UserView } | { type: 'Group', view: GroupView }

	const qName = url.searchParams.get('gname')
	const qId = url.searchParams.get('gid')
	if (qName && qId) {
		destination = {
			type: 'Group',
			view: {
				id: qId,
				title: qName,
				members: [] // not required for purposes of destionation
			}
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
		const result = await api.createTicket(newId, { title: topic, body: description, destination: { type: destination.type, id: destination.view.id }})
		// TODO: handle error
		if (result.status === 'Success') {
			goto(`/tickets/${newId}`)
		} else {
			console.error(result.payload)
		}
  }

	export let data: PageData

	async function searchFunction(keyword: string) {
		const api = new Api(fetch)
		const promises: [Promise<ApiResult<SearchResults<UserView>>>, Promise<ApiResult<SearchResults<GroupView>>>]
			= [api.searchUsers(keyword), api.searchGroups(keyword)]
		let options: ({ type: 'User', view: UserView } | { type: 'Group', view: GroupView })[] = []
		try {
			const [usrRes, grpRes] = await Promise.all(promises)
			if (usrRes.status === 'Success') {
				options = options.concat(usrRes.payload.top_hits.map(item => ({ type: 'User', view: item.value })))
			} else {
				// TODO: error handling
				console.error(usrRes.payload)
			}
			if (grpRes.status === 'Success') {
				options = options.concat(grpRes.payload.top_hits.map(item => ({ type: 'Group', view: item.value })))
			} else {
				// TODO: error handling
				console.error(usrRes.payload)
			}
		} catch (error) {
			// TODO: error handling
			console.error(error)
		}
		return options
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
			<AutoComplete
				{searchFunction}
				placeholder="Dorm,  IT,  319"
				bind:selectedItem={destination}
				class="my-1"
				inputClass="w-full"
				required
				localFiltering={false}
				labelFunction={(item) => item.type === 'User' ? item.view.name : item.view.title}
			>
				<div
					slot="item"
					let:item={item}
					let:label={label}
				>
					{#if item.type === 'Group'}
						{item.view.title}
					{:else}
						{item.view.name}
					{/if}
				</div>
			</AutoComplete>
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

<script lang="ts">
	import type {GroupProfileView, TicketListingViewExpandedItem, TicketStatus, UserProfileView} from 'backend'
	import TicketListItem from './TicketListItem.svelte'
	import { Button, Label, Popover } from 'flowbite-svelte'
	import { AutoComplete } from '$lib'
	import StatusBadge from '../StatusBadge.svelte';
	import Icon from '@iconify/svelte';
	import UserProfile from '../UserProfile/UserProfile.svelte';

	export let tickets: TicketListingViewExpandedItem[]
	export let users: Record<string, UserProfileView>
	export let groups: Record<string, GroupProfileView>

	export let displaySubmitter: boolean = false

	type Filters = {
		statuses: TicketStatus[]
		owners: UserProfileView[]
	}

	$: filters = { statuses: [], owners: [] } as Filters
	$: filteredTickets = tickets.filter(ticket => {
		if (filters.statuses.length > 0) {
			if (!filters.statuses.includes(ticket.status)) {
				return false
			}
		}
		if (filters.owners.length > 0) {
			if (!filters.owners.some(ow => ow.id === ticket.owner)) {
				return false
			}
		}
		return true
	})

	const resetFilters = () => {
		filters = { statuses: [], owners: [] }
	}

	const statuses: TicketStatus[] = ['Pending', 'InProgress', 'Declined', 'Fixed']
	$: selectedStatus = undefined as TicketStatus | undefined
	$: notSelectedStatuses = statuses.filter(st => !filters.statuses.includes(st))
	// get list of unique owner ids, attach user names and filter out any missing users
	const owners = [...new Set(tickets.map(t => t.owner))].map(ow => users[ow]).filter(ow => !!ow) as UserProfileView[]
	$: selectedOwner = undefined as UserProfileView | undefined
	$: notSelectedOwners = owners.filter(ow => !filters.owners.some(fow => fow.id === ow.id))

	const onStatusSelectChange = () => {
		if (!selectedStatus) return

		filters.statuses = [...filters.statuses, selectedStatus]
		selectedStatus = undefined
		const autocompleteClearButtons = document.querySelectorAll('span.autocomplete-clear-button');
      if (autocompleteClearButtons[0]) {
				// @ts-ignore
        autocompleteClearButtons[0].click();
			}
	}
	const onSelectedStatusRemove = (status: TicketStatus) => {
		filters.statuses = filters.statuses.filter(st => st !== status)
	}
	const onOwnerSelectChange = () => {
		if (!selectedOwner) return
		
		filters.owners = [...filters.owners, selectedOwner]
		selectedOwner = undefined
		const autocompleteClearButtons = document.querySelectorAll('span.autocomplete-clear-button');
      if (autocompleteClearButtons[1]) {
				// @ts-ignore
        autocompleteClearButtons[1].click();
			}
	}
	const onSelectedOwnerRemove = (id: string) => {
		filters.owners = filters.owners.filter(st => st.id !== id)
	}
</script>

<div class="flex flex-col gap-2 sm:gap-0">
	<div class="max-sm:hidden flex whitespace-nowrap text-xs uppercase text-gray-700 bg-gray-50 font-semibold rounded-t-md border-x border-t">
		<div class="w-32 px-6 py-3">Status</div>
		<div class="w-48 py-3">Submitted To</div>
		{#if displaySubmitter}
			<div class="w-48 py-3">Requested By</div>
		{/if}
		<div class="w-fit py-3">Topic</div>
		<div class="ml-8 flex items-center">
			<Button class="py-0.5 px-1.5" outline size='xs'>
				Filter
			</Button>
			<Popover
				trigger="click"
				placement="bottom"
			>
				<div class="min-w-[120px] normal-case font-medium text-xs flex flex-col pb-3">
					<Label class="flex flex-col">
						Status
						<AutoComplete
							class="my-1"
							inputClass="w-full items-center"
							items={notSelectedStatuses}
							bind:selectedItem={selectedStatus}
							onChange={onStatusSelectChange}
							showClear
						/>
					</Label>
					{#if filters.statuses.length > 0}
						<div class="mt-3 flex gap-1">
							{#each filters.statuses as status}
								<StatusBadge
									class="flex items-center"
									{status}
								>
									<button
										class="flex items-center w-2 h-2 ml-2"
										on:click={() => onSelectedStatusRemove(status)}
									>
										<Icon icon="fa:remove" style="color: black" />
									</button>
								</StatusBadge>
							{/each}
						</div>
					{/if}

					{#if displaySubmitter}
						<Label class="flex flex-col mt-4">
							Owner
							<AutoComplete
								class="my-1"
								inputClass="w-full items-center"
								items={notSelectedOwners}
								labelFieldName="name"
								bind:selectedItem={selectedOwner}
								onChange={onOwnerSelectChange}
								showClear
							/>
						</Label>
						{#if filters.owners.length > 0}
							<div class="mt-3 flex flex-col gap-1">
								{#each filters.owners as { id, name }}
									<div class="flex items-center border border-gray-500 text-gray-700 rounded-full p-1 px-2 w-fit">
										{name}
										<button
											class="flex items-center w-2 h-2 ml-2"
											on:click={() => onSelectedOwnerRemove(id)}
										>
											<Icon icon="fa:remove" style="color: black" />
										</button>
									</div>
								{/each}
							</div>
						{/if}
					{/if}

				</div>
			</Popover>
		</div>
	</div>
	{#if filteredTickets.length === 0}
		<div class="max-sm:hidden p-2 flex items-center border-x border-b rounded-b-md text-left">
			No items satisfy the filter conditions,
			<button
				class="ml-1 font-semibold hover:text-primary-700 transition"
				on:click={resetFilters}
			>
				reset filters
		</button>
		</div>
	{:else}
		{#each filteredTickets as ticket}
			<TicketListItem {ticket} {users} {groups} {displaySubmitter} />
		{/each}
	{/if}
</div>

<style>
	:global(span.autocomplete-clear-button) {
		width: 1px;
		height: 1px;
		overflow: hidden;
		padding: 0 !important;
		margin: 0;
		opacity: 0;
		cursor: default !important;
	}
</style>

<script lang="ts">
  import type { TicketListingViewExpandedItem } from 'backend'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import { goto } from '$app/navigation'
  import type {GroupProfileView, UserProfileView} from "backend";

  export let ticket: TicketListingViewExpandedItem
  export let users: Record<string, UserProfileView> // i hate ts i hate ts i hate ts
  export let groups: Record<string, GroupProfileView>

  export let displaySubmitter: boolean = false

  $: destination = ticket.destination.type === 'Group'
    ? { type: 'Group', id: ticket.destination.id, title: groups[ticket.destination.id]?.title ?? ticket.destination.id}
    : { type: 'User', id: ticket.destination.id, title: users[ticket.destination.id]?.name ?? ticket.destination.id}
  $: ticketOwner = users[ticket.owner]?.name || 'Unknown user'

  const handleTicketClick = () => {
    goto(`/tickets/${ticket.id}`)
  }
  const handleDestinationClick = () => {
    if (destination.type === 'Group') {
      goto(`/groups/${destination.id}`)
    } else {
      goto(`/users/${destination.id}`)
    }
  }
  const handleOwnerClick = () => {
    goto(`/users/${ticket.owner}`)
  }
</script>

<button on:click={handleTicketClick} class="hover:bg-gray-50 focus:bg-gray-50 text-left sm:hidden">
  <div
    class="grid grid-cols-[1fr_2fr] border rounded-md relative"
  >
    <div class="whitespace-nowrap text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5 rounded-tl-md">Submitted To</div>
    <button
      class="px-3 py-2 text-sm text-slate-500 text-left"
      on:click|stopPropagation={handleDestinationClick}
    >
      {destination.title}
      <StatusBadge status={ticket.status}/>
    </button>

    {#if displaySubmitter}
      <div class="whitespace-nowrap text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5 rounded-tl-md">Requested By</div>
      <button
        class="px-3 py-2 text-sm text-slate-500 text-left"
        on:click|stopPropagation={handleOwnerClick}
      >
        {ticketOwner}
      </button>
    {/if}

    <div class="text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5 rounded-bl-md">Topic</div>
    <div class="text-base px-3 py-2 break">
      {ticket.title}
    </div>
  </div>
</button>

<button on:click={handleTicketClick} class="max-sm:hidden hover:bg-gray-50 focus:bg-gray-50 border-x last:border-b last:rounded-b-md text-left">
  <div
    class="flex py-4 items-center rounded-b-md"
  >
    <div class="w-28 px-3 mr-4">
      <StatusBadge status={ticket.status}/>
    </div>
    <button
      class="w-48 text-sm text-slate-500 text-left break"
      on:click|stopPropagation={handleDestinationClick}
    >
      {destination.title}
    </button>
    {#if displaySubmitter}
      <button
        class="w-48 text-sm text-slate-500 text-left break"
        on:click|stopPropagation={handleOwnerClick}
      >
        {ticketOwner}
      </button>
    {/if}
    <div class="text-base break hover:cursor-pointer">
      {ticket.title}
    </div>
  </div>
</button>


<style>
  /* Unfortunately, tailwind does not have this */
  .break {
    word-break: break-word;
  }
</style>

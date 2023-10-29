<script lang="ts">
	import type { TicketListingViewExpandedItem } from 'backend'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
	import { goto } from '$app/navigation'

  export let ticket: TicketListingViewExpandedItem
  export let users: Map<string, string>
	export let groups: Map<string, string>

  let destination: string
  // @ts-ignore
  if (ticket.destination.Group) {
    // @ts-ignore
    destination = groups.get(ticket.destination.Group) || 'No-one'
  } else {
    // @ts-ignore
    destination = users.get(ticket.destination.User) || 'No-one'
  }

  const handleTicketClick = () => {
    goto(`/tickets/${ticket.id}`)
  }
  const handleDestinationClick = () => {
    if (ticket.destination.Group) {
      goto(`/groups/${destination}`)
    } else {
      goto(`/users/${destination}`)
    }
  }
</script>

<div
  class="grid grid-cols-[1fr_2fr] border rounded-md relative hover:bg-gray-50 sm:hidden"
  on:click={handleTicketClick}
>
  <div class="whitespace-nowrap text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5 rounded-tl-md">Submitted To</div>
  <div
    class="px-3 py-2 text-sm text-slate-500"
    on:click|stopPropagation={handleDestinationClick}
  >
    {destination}
    <StatusBadge status={ticket.status}/>
  </div>

  <div class="text-xs uppercase text-gray-700 bg-gray-50 font-semibold p-2.5 rounded-bl-md">Topic</div>
  <div class="text-base px-3 py-2 break">
    {ticket.title}
  </div>

</div>

<div
  class="max-sm:hidden flex py-4 items-center border-x hover:bg-gray-50 last:rounded-b-md"
  on:click={handleTicketClick}
>
  <div class="w-20 px-3 mr-4">
    <StatusBadge status={ticket.status}/>
  </div>
  <div
    class="w-60 px-6 text-sm text-slate-500 hover:cursor-pointer"
    on:click|stopPropagation={handleDestinationClick}
  >
    {destination}
  </div>
  <div class="text-base break">
    {ticket.title}
  </div>
</div>

<style>
  /* Unfortunately, tailwind does not have this */
  .break {
    word-break: break-word;
  }
</style>

<script lang="ts">
  import type { TicketListingViewExpandedItem } from 'backend'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import { goto } from '$app/navigation'
  import type {GroupProfileView, UserProfileView} from "backend";

  export let ticket: TicketListingViewExpandedItem
  export let users: Record<string, UserProfileView> // i hate ts i hate ts i hate ts
  export let groups: Record<string, GroupProfileView>

  let destination: { type: "Group" | "User"; id: string; title: string }
  if (ticket.destination.type === 'Group') {
    const g = groups[ticket.destination.id];
    destination = {
      type: 'Group',
      id: ticket.destination.id,
      title: g?.title ?? ticket.destination.id
    }
  } else {
    const u = users[ticket.destination.id];
    destination = {
      type: 'User',
      id: ticket.destination.id,
      title: u?.name ?? ticket.destination.id
    }
  }

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
  class="max-sm:hidden flex py-4 items-center border-x hover:bg-gray-50 last:rounded-b-md last:border-b"
  on:click={handleTicketClick}
>
  <div class="w-28 px-3 mr-4">
    <StatusBadge status={ticket.status}/>
  </div>
  <div
    class="w-60 px-6 text-sm text-slate-500 hover:cursor-pointer"
    on:click|stopPropagation={handleDestinationClick}
  >
    {destination.title}
  </div>
  <div class="text-base break hover:cursor-pointer">
    {ticket.title}
  </div>
</div>

<style>
  /* Unfortunately, tailwind does not have this */
  .break {
    word-break: break-word;
  }
</style>

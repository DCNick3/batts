<script lang="ts">
	import type { PageData } from './$types'
  import { TicketList } from '$lib/components/TicketList'
  import A from '$lib/components/A.svelte'

  export let data: PageData
</script>

{#if data.group === null}
  <h1 class="text-2xl font-semibold text-center">An error occured</h1>
{:else}
  <h1 class="text-2xl font-semibold text-center">Tickets of <A href={`/groups/${data.group.id}`}>{data.group.title}</A></h1>
  {#if data.groupTickets.payload.length > 0}
    <TicketList
      tickets={data.groupTickets.payload}
      users={data.groupTickets.users}
      groups={data.groupTickets.groups}
    />
  {:else}
    <h2 class="mx-auto mt-10 text-lg font-medium">This group does not have any tickets yet.</h2>
  {/if}
{/if}

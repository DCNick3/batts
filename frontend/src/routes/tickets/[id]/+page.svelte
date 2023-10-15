<script lang="ts">
  import TicketView from './TicketView.svelte'
  import type { PageData } from './$types'
  import { afterNavigate } from '$app/navigation'

  // afterNavigate(n => {
  //   n.from.
  // })

  export let data: PageData
</script>

{#if data.status === "Success"}
  <TicketView
    ticketView={data.payload}
    ticketId={data.ticketId}
    users={data.users}
    class="mb-10"
  />
{:else}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>{data.payload.report}. <a class="visited:text-primary-700 text-primary-500" on:click={() => window.history.back()} href={document.referrer}>Go back</a></p>
{/if}


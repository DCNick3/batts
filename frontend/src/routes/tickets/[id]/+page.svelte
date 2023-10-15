<script lang="ts">
  import TicketView from './TicketView.svelte'
  import type { PageData } from './$types'

  export let data: PageData
</script>

{#if data.status === "Success"}
  <TicketView
    ticketView={data.payload}
    ticketId={data.ticketId}
    users={data.users}
    class="mb-10"
  />
{:else if data.status === "Error"}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>{data.payload.report}. <a class="visited:text-primary-700 text-primary-500" on:click={() => window.history.back()} href={document.referrer}>Go back</a></p>
{:else}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>Could not connect to the server. <a class="visited:text-primary-700 text-primary-500" on:click={() => window.history.back()} href={document.referrer}>Go back</a></p>
{/if}


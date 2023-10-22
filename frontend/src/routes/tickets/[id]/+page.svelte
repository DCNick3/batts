<script lang="ts">
  import TicketView from './TicketView.svelte'
  import type { PageData } from './$types'
  import A from '$lib/components/A.svelte';

  export let data: PageData
</script>

{#if data.status === "Success"}
  <TicketView
    ticketView={data.payload}
    ticketId={data.ticketId}
    users={data.users}
    editPermissions={data.editPermissions}
    destination={data.destinationField}
    groupMembers={data.groupMembers}
  />
{:else if data.status === "Error"}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>{data.payload.report}. <A on:click={() => window.history.back()} href="/">Go back</A></p>
{:else}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>Could not connect to the server. <A on:click={() => window.history.back()} href="/">Go back</A></p>
{/if}


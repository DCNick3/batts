<script lang="ts">
  import TicketView from './TicketView.svelte'
  import type { PageData } from './$types'
  import A from '$lib/components/A.svelte';

  export let data: PageData
</script>

<svelte:head>
  {#if data.status === 'Success'}
    <title>{data.ticket.title}</title>
  {/if}
</svelte:head>

{#if data.status === "Success"}
  <TicketView
    ticketView={data.ticket}
    ticketId={data.ticketId}
    users={data.users}
    groups={data.groups}
    editPermissions={data.editPermissions}
  />
{:else if data.status === "Error"}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>{data.payload.report}. <A href="/">Go to main</A></p>
{:else}
  <!-- Inform user on error -->
  <h1 class="text-2xl font-semibold text-center">Could not load ticket</h1>
  <p>Could not connect to the server. <A href="/">Go to main</A></p>
{/if}

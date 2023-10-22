<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  import { Api } from 'backend'
  import type { TicketViewContent, UserView, TicketStatus } from 'backend'
  import { Timeline } from '$lib/components/Timeline'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import Ticket from './Ticket.svelte'
  import { Button, Textarea } from 'flowbite-svelte'
  import { invalidateAll } from '$app/navigation'
  import { getContext } from 'svelte'
  import A from '$lib/components/A.svelte'
  import Settings from '$lib/assets/Settings.svelte'

  export let ticketView: TicketViewContent
  export let ticketId: string
  export let users: Map<string, string>
  export let editPermissions: Set<string>
  export let destination: string

  type State = 'Sending' | 'Ok' | 'Error'
  let state: State = 'Ok'
  let messageField: string = ''
  let errorMessage: string = ''

  const user = getContext<SvelteStore<null | UserView>>('user')

  const submit = async () => {
    const message = messageField
    messageField = ''
		const api = new Api(fetch)
    state = 'Sending'
    try {
      const result = await api.sendTicketMessage(ticketId, { body: message })
      // TODO: receive more data from backend
      if (result.status === 'Success') {
        state = 'Ok'
        invalidateAll()
      } else {
        state = 'Error'
        errorMessage = result.payload.report
        messageField = message
      }
    } catch (error) {
      // TODO check error content
      state = 'Error'
      errorMessage = 'Connection failure'
      messageField = message
    }
  }

  const handleStatusChange = async (status: TicketStatus) => {
		const api = new Api(fetch)
    try {
      const result = await api.changeTicketStatus(ticketId, status)
      if (result.status === 'Success') {
        state = 'Ok'
        invalidateAll()
      } else {
        // TODO check error payload
        state = 'Error'
        errorMessage = 'Failed to update status'
      }
    } catch (error) {
      // TODO error handling
      console.error(error)
    }

  }

  const canEdit: boolean = $user !== null && editPermissions.has($user.id)
</script>

<!--
    A grid with two columns:
    Main body and status column
  -->
<div class={twMerge("mb-10 sm:mx-10 flex flex-col sm:grid sm:grid-cols-[3fr_1fr] gap-y-4 sm:gap-y-10 gap-x-14", $$props.class)}>
  <!-- Status view for mobile devices -->
  <div class="sm:hidden grid grid-cols-2 mb-4">
      <div class="font-semibold text-zinc-600">Submitted To</div>
      <div class="font-normal text-sm">{ticketView.destination}</div>  

      <div class="font-semibold text-zinc-600">Requested By</div>
      <div class="font-normal text-sm">{users.get(ticketView.owner) || "Unknown User"}</div>

      <div class="font-semibold text-zinc-600">Status</div>
      <div>
        <StatusBadge status={ticketView.status} />
      </div>
  </div>

  <!-- Status column -->
  <div class="max-sm:hidden flex flex-col gap-2 sm:gap-6 basis-1/4 sm:order-4">
    <div>
      {#if canEdit}
        <details>
          <summary class="list-none flex items-center gap-6 text-zinc-600 hover:text-primary-700 transition">
            <div class="font-semibold">Assigned To</div>
            <Settings />
          </summary>
          <div class="absolute z-50 bg-white p-2">
            Set assignee
            <div class="flex flex-col">
            </div>
          </div>
        </details>
      {:else}
        <div class="font-semibold text-zinc-600">Assigned To</div>
      {/if}
      <div class="font-normal text-sm">{ticketView.assignee ? users.get(ticketView.assignee) : 'No-one'}</div>  
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Submitted To</div>
      <div class="font-normal text-sm">{destination}</div>  
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Requested By</div>
      <div class="font-normal text-sm">{users.get(ticketView.owner) || "Unknown User"}</div>
    </div>

    <div>
      {#if canEdit}
        <details>
          <summary class="list-none flex items-center gap-6 text-zinc-600 hover:text-primary-700 transition hover:cursor-pointer">
            <div class="font-semibold">Status</div>
            <Settings />
          </summary>
          <div class="absolute z-50 bg-white p-2 font-semibold text-sm border rounded-sm">
            Set status to
            <div class="flex flex-col gap-1 font-normal text-sm mt-1">
              <button on:click={() => handleStatusChange('Pending')}>
                <StatusBadge status="Pending" class="hover:cursor-pointer w-full" />
              </button>
              <button on:click={() => handleStatusChange('InProgress')}>
                <StatusBadge status="In progress" class="hover:cursor-pointer w-full" />
              </button>
              <button on:click={() => handleStatusChange('Fixed')}>
                <StatusBadge status="Fixed" class="hover:cursor-pointer w-full" />
              </button>
              <button on:click={() => handleStatusChange('Declined')}>
                <StatusBadge status="Declined" class="hover:cursor-pointer w-full" />
              </button>
              </div>
          </div>
        </details>
      {:else}
        <div class="font-semibold text-zinc-600">Status</div>
      {/if}
      <StatusBadge status={ticketView.status} />
    </div>
  </div>

  <!-- Title -->
  <h1 class="text-2xl font-semibold text-center">{ticketView.title}</h1>

  <!-- For alignment -->
  <div class="max-sm:hidden"></div>

  <!-- Main body -->
  <div class="flex flex-col items-center sm:gap-4 basis-3/4">  
    <Timeline class="w-full">
      {#each ticketView.timeline as item}
        <Ticket item={item} users={users} />
      {/each}
    </Timeline>
    <!-- Only logged-in users may write messages -->
    {#if $user === null}
      <h1><A href="/login">Log in</A> to send messages.</h1>
    {:else}
      <form
        on:submit|preventDefault={submit}
        class="w-full gap-4"
      >
        {#if state === 'Error'}
          <span class="text-red-500">{errorMessage}</span>
        {/if}
        <Textarea
          class="mt-2 resize-none"
          name="message"
          rows=4
          placeholder="Write a message"
          bind:value={messageField}
          disabled={state === 'Sending'}
          required
        />
        <Button
          class="w-full"
          type="submit"
          disabled={state === 'Sending'}
        >
          {state === 'Sending' ? 'Sending' : 'Send message'}
        </Button>
      </form>
    {/if}
  </div>

</div>
<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  import { Api } from 'backend'
  import type { TicketViewContent, UserView } from 'backend'
  import { Timeline } from '$lib/components/Timeline'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import Ticket from './Ticket.svelte'
  import { Button, Textarea } from 'flowbite-svelte'
  import { invalidateAll } from '$app/navigation'
  import { getContext } from 'svelte'
  import A from '$lib/components/A.svelte'

  export let ticketView: TicketViewContent
  export let ticketId: string
  export let users: Map<string, string>

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
</script>

<!--
    A grid with two columns:
    Main body and status column
  -->
<div class={twMerge("mb-10 sm:mx-10 flex flex-col sm:grid sm:grid-cols-[3fr_1fr] gap-y-4 sm:gap-y-10 gap-x-14", $$props.class)}>
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
      <div class="font-semibold text-zinc-600">Submitted To</div>
      <div class="font-normal text-sm">{ticketView.destination}</div>  
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Requested By</div>
      <div class="font-normal text-sm">{users.get(ticketView.owner) || "Unknown User"}</div>
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Status</div>
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
          <span class="text-red-500">Failed to send message: {errorMessage}</span>
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
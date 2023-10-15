<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  import { Api } from 'backend'
  import type { TicketView } from 'backend'
  import { Timeline } from '$lib/components/Timeline'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import Ticket from './Ticket.svelte'
  import { Button, Textarea } from 'flowbite-svelte'

  export let ticketView: TicketView
  export let ticketId: string
  export let users: Map<string, string>

  type State = 'Sending' | 'Ok' | 'Error'
  let state: State = 'Ok'
  let messageField: string = ''
  let errorMessage: string = ''

  const submit = async () => {
    const message = messageField
    messageField = ''
		const api = new Api(fetch)
    state = 'Sending'
    try {
      const result = await api.sendTicketMessage(ticketId, { body: message })
      // TODO: receive more data from backend
      if (result.status === 'Success') {
        ticketView.timeline.push({
          date: (new Date()).toString(),
          content: {
            type: 'Message',
            from: '',
            text: message
          }
        })
        state = 'Ok'
        ticketView = ticketView
      } else {
        state = 'Error'
        errorMessage = result.payload.report
        messageField = message
      }
    } catch (error) {
      // TODO check error content
      state = 'Error'
      errorMessage = 'Connection failure'
    }
  }
</script>

<!--
    A grid with two columns:
    Main body and status column
  -->
<div class={twMerge("mx-10 grid grid-cols-[3fr_1fr] gap-y-10 gap-x-14", $$props.class)}>
  <!-- Title -->
  <h1 class="text-2xl font-semibold text-center">{ticketView.title}</h1>

  <!-- For alignment -->
  <div></div>

  <!-- Main body -->
  <div class="flex flex-col items-center gap-4 basis-3/4">  
    <Timeline class="w-full">
      {#each ticketView.timeline as item}
        <Ticket item={item} users={users} />
      {/each}
    </Timeline>
    <form
      on:submit|preventDefault={submit}
      class="w-full gap-4"
    >
      {#if state === 'Error'}
        <span>Failed to send message: {errorMessage}</span>
      {/if}
      <Textarea
        class="mt-2 resize-none"
        name="message"
        rows=4
        placeholder="Write a message"
        bind:value={messageField}
        disabled={state === 'Sending'}
      />
      <Button
        class="w-full"
        type="submit"
        disabled={state === 'Sending'}
      >
        {state === 'Sending' ? 'Sending' : 'Send message'}
      </Button>
    </form>
  </div>

  <!-- Status column -->
  <div class="flex flex-col gap-6 basis-1/4">
    <div>
      <div class="font-semibold text-zinc-600">Submitted To</div>
      <div class="font-normal text-sm">TODO</div>  
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
</div>
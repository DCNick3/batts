<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  import { Api } from 'backend'
  import type { TicketView, UserView, TicketStatus, UserId, UserProfileView, GroupId, GroupProfileView } from 'backend'
  import { Timeline } from '$lib/components/Timeline'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import Ticket from './Ticket.svelte'
  import { Button, Label, Textarea } from 'flowbite-svelte'
  import { DropdownItem } from 'flowbite-svelte'
  import { invalidateAll } from '$app/navigation'
  import { getContext } from 'svelte'
  import A from '$lib/components/A.svelte'
  import StatusOption from './StatusOption.svelte'
  import Icon from '@iconify/svelte'

  export let ticketView: TicketView
  export let ticketId: string
  export let users: Record<UserId, UserProfileView>
  export let groups: Record<GroupId, GroupProfileView>
  export let editPermissions: Set<string>

  $: getUsr = (id: UserId) => {
    const usr = users[id]
    if (usr) {
      return usr.name
    } else {
      return null
    }
  }
  $: getGrp = (id: GroupId) => {
    const grp = groups[id]
    if (grp) {
      return grp.title
    } else {
      return null
    }
  }

  type State = 'Sending' | 'Ok' | 'Error'
  let state: State = 'Ok'
  let messageField: string = ''
  let errorMessage: string = ''
  let files: FileList | undefined
  const ticketStatuses: TicketStatus[] = ['Pending', 'InProgress', 'Fixed', 'Declined']

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

  const handleSetAssignee = async (assigneeId: string | null) => {
    const api = new Api(fetch)
    try {
      const result = await api.changeTicketAssignee(ticketId, assigneeId)
      if (result.status === 'Success') {
        state = 'Ok'
        invalidateAll()
      } else {
        // TODO check error payload
        state = 'Error'
        errorMessage = 'Failed to assign ticket'
      }
    } catch (error) {
      // TODO error handling
      console.error(error)
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

  $: canEdit = $user !== null && editPermissions.has($user.id)

  const removeFiles = () => {
    files = undefined
  }
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
      <div class="font-normal text-sm">{getUsr(ticketView.owner) || "Unknown User"}</div>

      <div class="font-semibold text-zinc-600">Status</div>
      <div>
        <StatusBadge status={ticketView.status} />
      </div>
  </div>

  <!-- Status column -->
  <div class="max-sm:hidden flex flex-col gap-2 sm:gap-6 basis-1/4 sm:order-4">
    {#if ticketView.destination.type === 'Group'}
      <div>
        <StatusOption
          canEdit={canEdit}
          title="Assigned To"
          header="Set assignee"
        >
          {#each editPermissions as id}
            {#if getUsr(id) !== null && ticketView.assignee !== id}
              <DropdownItem>
                <button
                  on:click={() => handleSetAssignee(id)}
                >
                  {getUsr(id)}
                </button>
              </DropdownItem>
            {/if}
          {/each}
          {#if ticketView.assignee !== null}
            <DropdownItem>
              <button
                on:click={() => handleSetAssignee(null)}
              >
                Remove Assignee
              </button>
            </DropdownItem>
          {/if}
        </StatusOption>
        <div class="font-normal text-sm">{getUsr(ticketView.assignee || '') || 'No-one'}</div>  
      </div>
    {/if}
    

    <div>
      <div class="font-semibold text-zinc-600">Submitted To</div>
      <div class="font-normal text-sm">
        {ticketView.destination.type === 'User' ? getUsr(ticketView.destination.id) : getGrp(ticketView.destination.id)}
      </div>
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Requested By</div>
      <div class="font-normal text-sm">{getUsr(ticketView.owner) || "Unknown User"}</div>
    </div>

    <div>
      <StatusOption
        canEdit={canEdit}
        title="Status"
        header="Set status to"
      >
        {#each ticketStatuses as status}
          {#if status !== ticketView.status}
            <DropdownItem>
              <button
                class="w-full"
                on:click={() => handleStatusChange(status)}
              >
                <StatusBadge status={status} class="hover:cursor-pointer w-full" />
              </button>
            </DropdownItem>
          {/if}
        {/each}
      </StatusOption>
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
        <div class="flex px-2 gap-2 text-sm font-medium text-gray-700">
          {#if files}
            {#each files as file}
              <span class="flex items-center">
                {file.name}
              </span>
            {/each}
            <button on:click={() => removeFiles()}>
              <Icon icon="fa:remove" style="width:8px; height: 8px;" />
            </button>
          {/if}
          <Label class="ml-auto cursor-pointer">
            <Icon icon="icomoon-free:attachment" style="width: 20px; height: 20px;"/>
            <input type="file" class="hidden" multiple bind:files />
          </Label>  
        </div>
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
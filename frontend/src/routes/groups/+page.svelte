<script lang="ts">
	import type { GroupView, TicketListingViewExpandedItem, WithGroupsAndUsers } from 'backend'
  import { getContext } from 'svelte'
  import A from '$lib/components/A.svelte'
  import { TicketList } from '$lib/components/TicketList'
	import type { Writable } from 'svelte/store'
	import { pushApiError, pushError } from '$lib'

  const userGroups = getContext<SvelteStore<GroupView[]>>('userGroups')
  export let data

  const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
  data.errors.forEach(error => {
    if (error.type === 'Api') {
			pushApiError(errorContext, error.error)
		} else {
			pushError(errorContext, error.error)
		}
  })

  type TicketData = WithGroupsAndUsers<TicketListingViewExpandedItem[]>
  $: groups = $userGroups.map(grp => [grp, data.groupTickets.get(grp.id)] as [GroupView, TicketData | undefined])

</script>


{#if $userGroups.length > 0}
  <div class="flex flex-col gap-6">
    <h1 class="text-2xl font-semibold text-center">Your Groups</h1>
    {#each groups as [group, grpData]}
      <div>
        <div class="mb-2">
          <A href={`/groups/${group.id}/tickets`}>{group.title}</A>
        </div>
        {#if !grpData}
          <span class="text-lg font-medium">A connection error occured :c</span>
        {:else if grpData.payload.length === 0}
          <span class="text-lg font-medium">This group does not have any tickets yet.</span>
        {:else}
          <TicketList
            tickets={grpData.payload}
            users={grpData.users}
            groups={grpData.groups}
          />
        {/if}
      </div>
    {/each}
  </div>
{:else}
  <h1 class="text-2xl font-semibold text-center">You have no groups</h1>
  <div>You can create a new group <A href="/me">in your profile</A>.</div>
{/if}

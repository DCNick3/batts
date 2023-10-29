<script lang="ts">
	import type { GroupView } from 'backend'
  import { getContext } from 'svelte'
  import A from '$lib/components/A.svelte'

  const userGroups = getContext<SvelteStore<GroupView[]>>('userGroups')
</script>


{#if $userGroups.length > 0}
  <h1 class="text-2xl font-semibold text-center">Your Groups</h1>
  {#each $userGroups as group}
    <div>
      <A href={`/groups/${group.id}/tickets`}>{group.title}</A>
    </div>
    <div>*Tickets*</div>
  {/each}
{:else}
  <h1 class="text-2xl font-semibold text-center">You have no groups</h1>
  <div>You can create a new group <A href="/me">in your profile</A>.</div>
{/if}

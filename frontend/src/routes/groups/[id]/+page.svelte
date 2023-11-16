<script lang="ts">
  import type { PageData } from './$types'
  import GroupPage from './GroupView.svelte'

  export let data: PageData
</script>

{#if data.groupInfo === null}
  <!-- TODO: throw 404 page? -->
  <div>Group Not Found</div>
{:else}
  <GroupPage
    group={data.groupInfo.view}
    groupUsers={data.groupInfo.users}
    curUser={data.user}
  />
{/if}

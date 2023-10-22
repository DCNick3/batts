<script lang="ts">
  import type { PageData } from './$types'

  export let data: PageData
  $: groupInfo = data.groupInfo
</script>

{#if groupInfo === null}
  <!-- TODO: throw 404 page? -->
  <div>Group Not Found</div>
{:else}
  <div>
    <h1 class="text-lg font-semibold mb-4">{groupInfo.title}</h1>
    <h2 class="text-base font-semibold text-gray-700 mb-2">Members:</h2>
    <div>
      {#if data.users !== undefined}
        {#each data.users.values() as user}
          <span>{user}</span>
        {/each}
      {/if}
    </div>
  </div>
{/if}

<script lang="ts">
  import type { PageData } from './$types'
  import A from '$lib/components/A.svelte'
  import { Button } from 'flowbite-svelte'
	import { goto } from '$app/navigation';

  export let data: PageData
  $: groupInfo = data.groupInfo

  const handleOpenTicket = () => {
    goto(`/?gname=${groupInfo?.title}&gid=${groupInfo?.id}`)
  }
</script>

{#if groupInfo === null}
  <!-- TODO: throw 404 page? -->
  <div>Group Not Found</div>
{:else}
  <div class="flex gap-14">
    <div>
      <h1 class="text-lg font-semibold mb-4">{groupInfo.title}</h1>
      <h2 class="text-base font-semibold text-gray-700 mb-1">Members:</h2>
      <div>
        {#if data.users !== undefined}
          {#each data.users.entries() as [uid, username]}
            <A href={`/users/${uid}`}>{username}</A>
          {/each}
        {/if}
      </div>
    </div>

    <div>
      <Button on:click={handleOpenTicket}>
        Open a ticket
      </Button>
    </div>
  </div>
{/if}

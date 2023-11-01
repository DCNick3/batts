<script lang="ts">
  import type { PageData } from './$types'
  import A from '$lib/components/A.svelte'
  import { Button, Dropdown, Input } from 'flowbite-svelte'
	import { goto, invalidateAll } from '$app/navigation'
  import { Api } from 'backend'

  export let data: PageData
  $: groupInfo = data.groupInfo

  const handleOpenTicket = () => {
    goto(`/?gname=${groupInfo?.title}&gid=${groupInfo?.id}`)
  }

  let addUsersOpen = false
  const handleOpenAddUser = () => {
    addUsersOpen = true
  }

  let userIdField: string
  const handleAddUser = async () => {
    if (groupInfo === null) return

    const api = new Api(fetch)
    try {
      const result = await api.addGroupMember(groupInfo.id, userIdField)
      if (result.status === 'Success') {
        invalidateAll()
        addUsersOpen = false
        userIdField = ''
      } else {
        // TODO: error handling
        console.error(result.payload)
      }
    } catch (error) {
      // TODO: error handling
      console.error(error)
    }
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
        {#if data.users !== null}
          {#each Object.entries(data.users) as [uid, profile]}
            <A href={`/users/${uid}`}>{profile.name}</A>
          {/each}
        {/if}
      </div>
    </div>

    <div>
      <Button on:click={handleOpenTicket}>
        Open a ticket
      </Button>
      {#if groupInfo.members.includes(data.user?.id || '')}
        <Button on:click={handleOpenAddUser}>
          Add User
        </Button>
        <Dropdown bind:open={addUsersOpen} class="p-2">
          <Input
            class="mb-2"
            bind:value={userIdField}
            placeholder="Enter user id"
            required
          />
          <Button on:click={handleAddUser} class="w-full">Add</Button>
        </Dropdown>
      {/if}
    </div>
  </div>
{/if}

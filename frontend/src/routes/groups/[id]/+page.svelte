<script lang="ts">
  import type { PageData } from './$types'
  import A from '$lib/components/A.svelte'
  import { Button, Dropdown, Input } from 'flowbite-svelte'
	import { beforeNavigate, goto, invalidateAll } from '$app/navigation'
  import { Api, type GroupView } from 'backend'
  import Settings from '$lib/assets/Settings.svelte'
  import type { Update } from './updates'
  import Icon from '@iconify/svelte'

  export let data: PageData
  $: groupInfo = data.groupInfo

  beforeNavigate(({ cancel }) => {
    const titleChanged = groupTitleField !== data.groupInfo?.title
    const changesPresent = titleChanged || updates.members.length > 0

    if (isEditMode && changesPresent) {
      if (!confirm('Are you sure you want to leave this page? You have unsaved changes that will be lost.')) {
        cancel()
      }
    }
  })

  const handleOpenTicket = () => {
    goto(`/?gname=${groupInfo?.title}&gid=${groupInfo?.id}`)
  }

  let groupTitleField: string = ''
  let isEditMode: boolean = false
  let updates: { members: Update[] } = { members: [] }
  let addUsersOpen = false
  let userIdField: string = ''

  if (data.groupInfo !== null) {
    groupTitleField = data.groupInfo.title
  }

  const handleEditClick = () => {
    isEditMode = true
  }
  // const handleSaveClick = () => {
  //   isEditMode = false
  // }

  const handleOpenAddUser = () => {
    addUsersOpen = true
  }

  const handleAddUser = () => {
    if (groupInfo === null) return

    // const api = new Api(fetch)
    // try {
    //   const result = await api.addGroupMember(groupInfo.id, userIdField)
    //   if (result.status === 'Success') {
    //     invalidateAll()
    //     addUsersOpen = false
    //     userIdField = ''
    //   } else {
    //     // TODO: error handling
    //     console.error(result.payload)
    //   }
    // } catch (error) {
    //   // TODO: error handling
    //   console.error(error)
    // }
    updates.members.push({ type: 'AddUser', id: userIdField})
    addUsersOpen = false
    userIdField = ''
  }
  const handleDeleteUser = (id: string) => {
    if (groupInfo === null) return

    updates.members.push({ type: 'DeleteUser', id })
  }

  const handleUpdates = async () => {
    if (groupInfo === null) return

    const titleChanged = groupTitleField !== groupInfo.title
    const changesPresent = titleChanged || updates.members.length > 0

    const api = new Api(fetch)
    try {
      const promises = updates.members.map(up => {
        if (up.type === 'AddUser') {
          return api.addGroupMember((groupInfo as GroupView).id, up.id)
        } else {
          // TODO: remove user
          return Promise.resolve()
        }
      })
      if (titleChanged) {
        // TODO: update group title
        // promises.push(api.updateTitle(groupInfo.id, groupTitle))
      }

      const results = await Promise.all(promises)
      results.forEach(res => {
        if (!res) return
        if (res.status == 'Error') {
          // TODO: error handling
          console.error(res.payload)
        }
      })
    } catch (error) {
      // TODO: error handling
      console.error(error)
    }

    isEditMode = false
  }
</script>

{#if groupInfo === null}
  <!-- TODO: throw 404 page? -->
  <div>Group Not Found</div>
{:else}
  <div class="flex gap-14 sm:gap-52">
    <div>
      <h1 class="text-lg font-semibold mb-4 flex items-center">
        {#if isEditMode}
          <Input bind:value={groupTitleField} />
        {:else}
          {groupInfo.title}
        {/if}
        {#if groupInfo.members.includes(data.user?.id || '')}
          {#if !isEditMode}
            <button class="ml-2" on:click={() => handleEditClick()}>
              <Settings />
            </button>
          {:else}
            <Button class="ml-2 p-1" on:click={() => handleUpdates()}>Save changes</Button>
          {/if}
        {/if}
      </h1>
      <h2 class="text-base font-semibold text-gray-700 mb-1">Members:</h2>
      <div class="flex flex-col">
        {#if data.users !== null}
          {#each Object.entries(data.users) as [uid, profile]}
            <div class="flex items-center gap-2">
              <A href={`/users/${uid}`}>{profile.name}</A>
              {#if isEditMode}
                <button on:click={() => handleDeleteUser(uid)}>
                  <Icon icon="fa:remove" style="color: red" />
                </button>                
              {/if}
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <div class="flex flex-col gap-2">
      <Button on:click={handleOpenTicket}>
        Open a ticket
      </Button>
      {#if isEditMode}
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
      {/if}
    </div>
  </div>
{/if}

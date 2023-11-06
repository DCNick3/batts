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
    goto(`/?gname=${data.groupInfo?.title}&gid=${data.groupInfo?.id}`)
  }

  $: getUsr = (id: string) => {
    const usr = data.groupUsers ? data.groupUsers[id] : undefined
    if (usr) {
      return usr.name
    } else {
      return null
    }
  }

  // During editting, will be updated to show user how
  // their updates affect the view
  $: groupTitle = data.groupInfo?.title
  $: groupMembers = data.groupInfo?.members || []

  let groupTitleField: string = data.groupInfo?.title || ''
  let isEditMode: boolean = false
  let updates: { members: Update[] } = { members: [] }
  let addUsersOpen = false
  let userIdField: string = ''
  const isGroupMember: boolean = data.groupInfo?.members.includes(data.user?.id || '') || false

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
    const uid = userIdField
    // Keep update before save
    updates.members.push({ type: 'AddUser', id: uid})
    addUsersOpen = false
    userIdField = ''

    // Update user view
    groupMembers.push(uid)
  }
  const handleDeleteUser = (id: string) => {
    // Keep update before save
    updates.members.push({ type: 'DeleteUser', id })

    // Update user view
    const index = groupMembers.indexOf(id)
    if (index > -1) { // only splice array when item is found
      groupMembers.splice(index, 1); // 2nd parameter means remove one item only
    }
    groupMembers = [...groupMembers]
  }

  const handleUpdates = async () => {
    const titleChanged = groupTitleField !== groupTitle
    const changesPresent = titleChanged || updates.members.length > 0

    const api = new Api(fetch)
    try {
      const promises = updates.members.map(up => {
        if (up.type === 'AddUser') {
          return api.addGroupMember(data.groupId, up.id)
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
    invalidateAll()
  }
</script>

{#if data.groupInfo === null}
  <!-- TODO: throw 404 page? -->
  <div>Group Not Found</div>
{:else}
  <div class="flex gap-14 sm:gap-52">
    <div>
      <h1 class="text-lg font-semibold mb-4 flex items-center">
        {#if isEditMode}
          <Input bind:value={groupTitleField} />
        {:else}
          {groupTitle}
        {/if}
        {#if isGroupMember}
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
        {#if groupMembers}
          <!-- {#each Object.entries(data.groupUsers) as [uid, profile]} -->
          {#each groupMembers as grpMemberId}
            <div class="flex items-center gap-2">
              <A href={`/users/${grpMemberId}`}>{getUsr(grpMemberId) || 'Unknown user'}</A>
              {#if isEditMode}
                <button on:click={() => handleDeleteUser(grpMemberId)}>
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
        {#if isGroupMember}
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

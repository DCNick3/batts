<script lang="ts">
  import { Api } from 'backend'
	import type { GroupView, UserProfileView, UserView } from 'backend'
	import { beforeNavigate, goto, invalidateAll } from '$app/navigation'
  import { Button, Dropdown, Input } from 'flowbite-svelte'
  import A from '$lib/components/A.svelte'
  import { Updates } from './updates'
  import Settings from '$lib/assets/Settings.svelte'
  import Icon from '@iconify/svelte'
  import { getContext } from 'svelte'
  import { UserSearch, pushApiError } from '$lib'
	import type { Writable } from 'svelte/store'

  export let group: GroupView
  export let groupUsers: Record<string, UserProfileView>
  export let curUser: UserView | null

  const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')

  const updates = new Updates()

  beforeNavigate(({ cancel }) => {
    const titleChanged = groupTitleField !== group.title
    const changesPresent = titleChanged || updates.updatesPresent()

    if (isEditMode && changesPresent) {
      if (!confirm('Are you sure you want to leave this page? You have unsaved changes that will be lost.')) {
        cancel()
      }
    }
  })

  const handleOpenTicket = () => {
    goto(`/?gname=${group.title}&gid=${group.id}`)
  }

  $: getUsr = (id: string) => {
    const usr = groupUsers[id]
    if (usr) {
      return usr.name
    } else {
      return null
    }
  }

  // During editting, will be updated to show user how
  // their updates affect the view
  $: groupTitle = group.title
  $: groupMembers = group.members || []
  // $: updates = { members: [] } as { members: Update[] }

  let groupTitleField: string = group.title || ''
  let isEditMode: boolean = false
  let addUsersOpen = false
  let selectedUser: UserView
  const isGroupMember: boolean = group.members.includes(curUser?.id || '') || false

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
    if (!selectedUser) return
  
    const uid = selectedUser.id
    // Keep update before save
    updates.add(uid)
    addUsersOpen = false
    // userIdField = ''

    groupUsers[uid] = selectedUser
    // Update user view
    groupMembers = [...groupMembers, uid]
  }
  const handleDeleteUser = (id: string) => {
    // Keep update before save
    updates.delete(id)

    // Update user view
    const index = groupMembers.indexOf(id)
    if (index > -1) { // only splice array when item is found
      groupMembers.splice(index, 1) // 2nd parameter means remove one item only
    }
    groupMembers = [...groupMembers]
  }

  const handleUpdates = async () => {
    const trimmedTitle = groupTitleField.trim()
    const titleChanged = trimmedTitle !== groupTitle

    const api = new Api(fetch)
    try {
      const promises = updates.getUpdates().map(up => {
        if (up.type === 'AddUser') {
          return api.addGroupMember(group.id, up.id)
        } else {
          return api.removeGroupMember(group.id, up.id)
        }
      })
      if (titleChanged) {
        promises.push(api.changeGroupTitle(group.id, groupTitleField))
      }

      const results = await Promise.all(promises)
      results.forEach(res => {
        if (!res) return
        if (res.status == 'Error') {
          console.error(res.payload)
          pushApiError(errorContext, res.payload)
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
        {#each groupMembers as grpMemberId}
          <div class="flex items-center gap-2">
            <A href={`/users/${grpMemberId}`}>{getUsr(grpMemberId) || 'Unknown user'}</A>
            {#if isEditMode && grpMemberId !== curUser?.id}
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
        <Dropdown bind:open={addUsersOpen} class="p-2 flex flex-col gap-2">
          <UserSearch
            class="mb-2"
            inputClass="w-full"
            dropdownClassName="z-50"
            bind:selectedUser={selectedUser}
            placeholder="Search users"
            localFiltering={true}
            itemFilterFunction={user => !groupMembers.includes(user.id)}
            required
          />
          <Button on:click={handleAddUser} class="w-full">Add</Button>
        </Dropdown>
      {/if}
    {/if}
  </div>
</div>
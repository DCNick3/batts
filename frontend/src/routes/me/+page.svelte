<script lang="ts">
  import tgIcon from '$lib/assets/telegram_icon_48.png'
  import { getContext } from 'svelte'
  import { Api, generateId } from 'backend'
	import type { UserView } from 'backend'
  import { Button, Input } from 'flowbite-svelte'
  import { goto, invalidateAll } from '$app/navigation'
	import type { PageData } from './$types'
  import { UserProfile } from '$lib/components/UserProfile'
	import type { Writable } from 'svelte/store'
	import { pushApiError, pushError } from '$lib'

  const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
  const user = getContext<SvelteStore<null | UserView>>('user')

  let groupName = ''
  export let data: PageData

  const handleCreateGroup = async () => {
    const api = new Api(fetch)

    try {
      const id = generateId()
      const result = await api.createGroup(id, { title: groupName })
      if (result.status === 'Success') {
        goto(`/groups/${id}`)
        groupName = ''
        invalidateAll()
      } else {
        console.error(result.payload)
        pushApiError(errorContext, result.payload)
      }
    } catch (error) {
      console.error(error)
      pushError(errorContext, { title: 'Unexpected error', message: (error as any)?.message || '' })
    }
  }
</script>

<svelte:head>
  <title>{$user === null ? 'User Profile' : $user.name}</title>
</svelte:head>

{#if $user === null}
  <!-- TODO: throw 404 page? -->
  <div>User Not Found</div>

{:else}

  <UserProfile
    isMe
    user={$user}
    groups={data.userGroups}
  >
    <svelte:fragment slot="first-col">
      <h1 class="mb-2 font-semibold text-xl">Connected accounts</h1>

      {#if $user?.identities.telegram !== null}
        <div class="self-start flex items-center gap-2">
          <img src={tgIcon} alt="Icon of an origami plane, telegram" />
          <span class="font-medium text-base">
            {$user?.identities.telegram.first_name + " " + $user?.identities.telegram.last_name}
          </span>
        </div>
      {/if}

      {#if $user?.identities.university !== null}
        <!-- TODO -->
        <div></div>
      {/if}
    </svelte:fragment>

    <svelte:fragment slot="second-col">
      <h1 class="mt-4 mb-4 font-semibold text-xl">Create a new group</h1>
      <form on:submit|preventDefault={handleCreateGroup}>
        <Input
          class="mb-2"
          bind:value={groupName}
          placeholder="Enter group name"
          required
        />
        <Button type="submit" class="text-base w-full">Create</Button>
      </form>
    </svelte:fragment>
  </UserProfile>

{/if}

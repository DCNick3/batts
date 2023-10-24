<script lang="ts">
  import tgIcon from '$lib/assets/telegram_icon_48.png'
  import { getContext } from 'svelte'
  import { Api, generateId } from 'backend'
	import type { UserView } from 'backend'
  import Avatar from '$lib/components/Avatar.svelte'
  import { Button, Input } from 'flowbite-svelte'
  import { goto } from '$app/navigation'
	import type { PageData } from './$types'
  import A from '$lib/components/A.svelte'

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

{#if $user === null}
  <!-- TODO: throw 404 page? -->
  <div>User Not Found</div>
{:else}
  <div class="flex gap-52">
    <div class="flex flex-col items-center">
      <Avatar
        class="w-56 h-56 mb-2"
        str={$user.id}
      />
      <span class="font-medium text-lg mb-6">{$user?.name}</span>

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
    </div>

    <div class="flex flex-col">
      <h1 class="mb-4 font-semibold text-xl">Your groups</h1>
      {#each data.groups as group}
        <A href={`/groups/${group.id}`}>
          {group.title}
        </A>
        <div>
        </div>
      {/each}
      <h1 class="mt-6 mb-4 font-semibold text-xl">Create a new group</h1>
      <form on:submit|preventDefault={handleCreateGroup}>
        <Input
          class="mb-2"
          bind:value={groupName}
          placeholder="Enter group name"
          required
        />
        <Button type="submit" class="text-base w-full">Create</Button>
      </form>
    </div>
  </div>

{/if}

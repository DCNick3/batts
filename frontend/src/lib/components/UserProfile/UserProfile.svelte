<script lang="ts">
  import type { UserProfileView, GroupViewContent } from 'backend'
	import Avatar from '../Avatar.svelte'
  import tgIcon from '$lib/assets/telegram_icon_48.png'
  import A from '../A.svelte'
  import { Button, Input } from 'flowbite-svelte'

  export let user: UserProfileView
  export let groups: GroupViewContent[]
</script>

<div class="flex gap-52">
  <div class="flex flex-col items-center">
    <Avatar
      class="w-56 h-56 mb-2"
      str={user.id}
    />
    <span class="font-medium text-lg mb-6">{user?.name}</span>

    <slot name="first-col"></slot>

    <!-- <h1 class="mb-2 font-semibold text-xl">Connected accounts</h1> -->

    <!-- {#if user.identities.telegram !== null}
      <div class="self-start flex items-center gap-2">
        <img src={tgIcon} alt="Icon of an origami plane, telegram" />
        <span class="font-medium text-base">
          {user?.identities.telegram.first_name + " " + user?.identities.telegram.last_name}
        </span>
      </div>
    {/if}

    {#if user?.identities.university !== null}
      <div></div>
    {/if} -->
  </div>

  <div class="flex flex-col">
    <h1 class="mb-4 font-semibold text-xl">Groups</h1>
    {#each groups as group}
      <A href={`/groups/${group.id}`}>
        {group.title}
      </A>
      <div>
      </div>
    {/each}
    <slot name="second-col"></slot>
    <!-- <h1 class="mt-6 mb-4 font-semibold text-xl">Create a new group</h1> -->
    <!-- <form on:submit|preventDefault={handleCreateGroup}>
      <Input
        class="mb-2"
        bind:value={groupName}
        placeholder="Enter group name"
        required
      />
      <Button type="submit" class="text-base w-full">Create</Button>
    </form> -->
  </div>
</div>

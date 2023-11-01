<script lang="ts">
  import type { UserProfileView, GroupView } from 'backend'
	import Avatar from '../Avatar.svelte'
  import tgIcon from '$lib/assets/telegram_icon_48.png'
  import A from '../A.svelte'
  import { Button, Input } from 'flowbite-svelte'

  export let user: UserProfileView
  export let groups: GroupView[]
</script>

<div class="flex gap-52">
  <div class="flex flex-col items-center">
    <Avatar
      class="w-56 h-56 mb-2"
      str={user.id}
    />
    <div class="flex flex-col items-center mb-6">
      <div class="font-medium text-lg">{user.name}</div>
      <div class="font-medium text-sm">id: {user.id}</div>  
    </div>

    <slot name="first-col"></slot>
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
  </div>
</div>

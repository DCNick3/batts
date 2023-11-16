<script lang="ts">
  import type { UserView } from 'backend'
  import Avatar from '../Avatar.svelte'

  export let user: UserView
  $: tgUsername = user.identities.telegram?.username
</script>

<div class="flex gap-2 items-center">
  <Avatar
    class="w-6 h-6 border border-primary-700"
    str={user.id}
  />
  <span>{user.name}</span>
  {#if tgUsername}
    <span class="text-xs text-gray-600">{tgUsername}</span>    
  {/if}
</div>

<script lang="ts">
  import tgIcon from '$lib/assets/telegram_icon_48.png'
  import { getContext } from 'svelte'
	import type { UserView } from 'backend';

  export const user = getContext<SvelteStore<null | UserView>>('user')
</script>

{#if $user === null}
  <!-- TODO: throw 404 page? -->
  <div>User Not Found</div>
{:else}
  <div class="flex flex-col">
    <div class="mb-10 flex items-center gap-4">
      <div class="w-14 h-14 rounded-full bg-amber-600"></div>
      <span class="font-medium text-lg">{$user?.name}</span>
    </div>

    <h1 class="mb-4 font-semibold text-xl">Connected accounts</h1>
    
    {#if $user?.identities.telegram !== null}
      <div class="flex items-center gap-2">
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
{/if}

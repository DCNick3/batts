<script lang="ts">
  import type { PageData } from './$types'
  import tgIcon from '$lib/assets/telegram_icon_48.png'

  export let data: PageData
</script>

<div class="flex flex-col">
  <div class="mb-10 flex items-center gap-4">
    <div class="w-14 h-14 rounded-full bg-amber-600"></div>
    <span class="font-medium text-lg">{data.name}</span>
  </div>

  <h1 class="mb-4 font-semibold text-xl">Connected accounts</h1>
  
  {#if data.identities.telegram !== null}
    <div class="flex items-center gap-2">
      <img src={tgIcon} alt="Icon of an origami plane, telegram" />
      <span class="font-medium text-base">
        {data.identities.telegram.first_name + " " + data.identities.telegram.last_name}
      </span>
    </div>
  {/if}

  {#if data.identities.university !== null}
    <!-- TODO -->
    <div></div>
  {/if}
</div>

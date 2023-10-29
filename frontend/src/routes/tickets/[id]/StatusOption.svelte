<script lang="ts">
  import { Dropdown, DropdownItem } from 'flowbite-svelte'
  import Settings from '$lib/assets/Settings.svelte'

  export let canEdit: boolean
  export let title: string
  export let header: string
</script>

{#if canEdit}
  <div>
    <div class="list-none flex items-center gap-6 text-zinc-600 hover:text-primary-700 transition hover:cursor-pointer">
      <div class="font-semibold">{title}</div>
      <Settings />
    </div>
    <Dropdown
      placement="bottom-start"
    >
      <div class="p-2 font-semibold text-sm">
        {header}
      </div>
      <slot />
    </Dropdown>
  </div>
{:else}
  <div class="font-semibold text-zinc-600">{title}</div>
{/if}
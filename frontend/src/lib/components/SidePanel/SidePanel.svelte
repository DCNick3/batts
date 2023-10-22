<script lang="ts">
  import { Drawer, CloseButton } from 'flowbite-svelte'
  import NavLink from '$lib/components/NavLink.svelte'
  import { sineIn } from 'svelte/easing'
  import { twMerge } from 'tailwind-merge'
  import Content from './Content.svelte'
  import Logo from '$lib/components/Logo.svelte'

  export let hidden = true

  let transitionParams = {
    x: -320,
    duration: 200,
    easing: sineIn
  }
  const hide = () => {
    hidden = true
  }
</script>

<Drawer
  bind:hidden={hidden}
  class={twMerge("sm:hidden p-0 w-fit bg-slate-50", $$props.class)}
  {transitionParams}
  backdrop={false}
>
  <div class="flex items-between py-2 px-4">
    <Logo class="w-8 h-8" />
    <CloseButton on:click={hide}/>
  </div>
  <Content click={hide} />
</Drawer>

<Content click={hide} class="max-sm:hidden sm:fixed sm:top-14 sm:bottom-0" />

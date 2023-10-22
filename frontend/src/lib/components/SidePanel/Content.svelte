<script lang="ts">
  import NavLink from "../NavLink.svelte"
  import { twMerge } from 'tailwind-merge'
  export let click: (() => void) | undefined = undefined
</script>

<div class={twMerge('w-64 h-full bg-slate-50 flex flex-col items-center gap-3 p-4 pt-2 sm:pt-8', $$props.class)}>
  <NavLink on:click={click} href="/">Open a Ticket</NavLink>
  <NavLink on:click={click} href="/assigned">Assigned Tickets</NavLink>  
</div>

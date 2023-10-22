<script lang="ts">
  import NavLink from "../NavLink.svelte"
  import { twMerge } from 'tailwind-merge'
</script>

<div class={twMerge('w-64 h-full bg-slate-50 flex flex-col items-center gap-3 p-4 pt-2 sm:pt-8', $$props.class)}>
  <NavLink href="/">Open a Ticket</NavLink>
  <NavLink href="/assigned">Assigned Tickets</NavLink>  
</div>

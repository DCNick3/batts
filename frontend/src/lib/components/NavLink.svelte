<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  export let href: string
</script>

<a
  class={twMerge("font-semibold visited:text-black hover:text-primary-700 transition duration-200", $$props.class)}
  href={href}
  on:click
>
  <slot />
</a>
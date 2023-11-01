<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  export let href: string

</script>

<a
  class={twMerge("visited:text-primary-700 text-primary-700 hover:text-primary-500 transition font-semibold", $$props.class)}
  on:click
  href={href}
>
  <slot />
</a>

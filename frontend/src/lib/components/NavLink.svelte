<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  export let href: string
</script>

<a
  class={twMerge("font-semibold hover:text-primary-700 transition duration-200", $$props.class)}
  href={href}
>
  <slot />
</a>
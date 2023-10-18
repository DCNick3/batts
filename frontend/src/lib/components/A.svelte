<script lang="ts">
  export let href: string
</script>

<a
  class="visited:text-primary-700 text-primary-500 font-semibold"
  on:click
  href={href}
>
  <slot />
</a>

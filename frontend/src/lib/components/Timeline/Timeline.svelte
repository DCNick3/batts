<script>import { twMerge } from 'tailwind-merge';
  import { setContext } from 'svelte';
  export let order = 'default';
  setContext('order', order);
  let olClass = 'relative'
  </script>
  
  <ol class={twMerge(olClass, $$props.class)}>
    <slot />
  </ol>

  <!--
  @component
  [Go to docs](https://flowbite-svelte.com/)
  ## Props
  @prop export let order: 'default' | 'vertical' | 'horizontal' | 'activity' | 'group' = 'default';
  -->

<script lang="ts">
  import { twMerge } from 'tailwind-merge'

  export let color = 'currentColor';
  export let size = '24'
  let buttonClass = 'focus:outline-none whitespace-normal rounded focus:ring-1 p-0.5 focus:ring-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'
  let svgpath = `<path stroke="${color}" stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"></path> `;

</script>

<button on:click type="button" {...$$restProps} class={twMerge(buttonClass, $$props.class)}>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    role="button"
    tabindex="0"
    width={size}
    height={size}
    class={$$props.class}
    fill="none"
    viewBox="0 0 24 24"
    stroke-width="2"
  >
    {@html svgpath}
  </svg>
</button>

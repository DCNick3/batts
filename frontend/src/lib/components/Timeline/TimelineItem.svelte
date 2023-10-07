<script>import { twMerge } from 'tailwind-merge';
  export let title = '';
  export let date = '';
  const liClass = 'mb-6 ml-4'
  const divClass = 'absolute w-3 h-3 bg-gray-200 rounded-full mt-1.5 -left-1.5 border border-white dark:border-gray-900 dark:bg-gray-700'
  const timeClass = 'ml-4 mb-1 text-sm font-normal leading-none text-gray-400 dark:text-gray-500'
  let liCls = twMerge(liClass, $$props.classLi);
  let divCls = twMerge(divClass, $$props.classDiv);
  let timeCls = twMerge(timeClass, $$props.classTime);
  const h3Cls = twMerge('ml-4 text-lg font-semibold text-gray-900 dark:text-white', $$props.classH3);
  </script>
  
  <li class={liCls}>
    <slot name="icon">
      <div class={divCls} />
    </slot>  
    {#if date}
      <time class={timeCls}>{date}</time>
    {/if}
  
    {#if title}
      <h3 class={h3Cls}>
        {title}
      </h3>
    {/if}
  
    <slot />
  </li>
  
  <!--
  @component
  [Go to docs](https://flowbite-svelte.com/)
  ## Props
  @prop export let title: string = '';
  @prop export let date: string = '';
  @prop export let svgClass: string = 'w-3 h-3 text-primary-600 dark:text-primary-400';
  -->
  
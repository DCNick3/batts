<script lang="ts" generics="T">
  	// @ts-ignore
	import AutoComplete from "simple-svelte-autocomplete"

  import { twMerge } from 'tailwind-merge'
	let defaultClass = 'text-sm p-2.5 text-gray-900 bg-gray-50 border border-gray-300 rounded-lg focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500';
	$: autocompleteClass = twMerge(defaultClass, $$props.class);

  // @ts-ignore
  export let items: T[]
  export let required = false
  export let placeholder="Choose option..."
  export let labelFieldName: string
  export let valueFieldName: string

	export let selectedItem: T
</script>

<AutoComplete
	className="block w-full my-1"
	class={autocompleteClass}
	items={items}
	bind:selectedItem
	labelFieldName={labelFieldName}
	valueFieldName={valueFieldName}
	placeholder={placeholder}
	hideArrow
	required={required}
/>

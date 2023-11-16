<script lang="ts" generics="T">
  // @ts-ignore
	import AutoComplete from 'simple-svelte-autocomplete'

  import { twMerge } from 'tailwind-merge'
	let defaultClass = 'text-sm p-2.5 text-gray-900 bg-gray-50 border border-gray-300 rounded-lg focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500';

  export let items: NonNullable<T>[] = []
  export let searchFunction: ((keyword: string) => Promise<T[]>) | undefined = undefined
  export let required = false
  export let placeholder = 'Choose option...'
	export let inputClass = ''
  export let labelFieldName: keyof NonNullable<T> | undefined = undefined
  export let valueFieldName: keyof NonNullable<T> | undefined = undefined
	export let labelFunction: ((item: T) => string) | undefined = undefined
	export let valueFunction: ((item: T) => any) | undefined = undefined
	export let onChange: any = undefined
	export let showClear = false
	export let readOnly = false
	export let localFiltering = true

	export let selectedItem: T
</script>

<AutoComplete
	className={twMerge("block w-full", $$props.class)}
	class={twMerge(defaultClass, inputClass)}
	dropdownClassName={"rounded mt-1 !border-gray-300 !bg-gray-50 !text-gray-900 !ring-primary-500"}
	{items}
  {searchFunction}
	bind:selectedItem
	{labelFieldName}
	{valueFieldName}
	{labelFunction}
	{valueFunction}
	{placeholder}
	hideArrow
	{readOnly}
	{showClear}
	{required}
	{onChange}
	{localFiltering}
>
	<slot
		name="item"
		slot="item"
		let:item={item}
		item={item}
		let:label={label}
		label={label}
	>
		{@html label}
	</slot>
</AutoComplete>

<style>
	:global(.autocomplete-list-item.selected) {
		background-color: #02b511 !important; /* primary-600 */
	}
	:global(.autocomplete-list-item.confirmed) {
		background-color: #43d964 !important; /* primary-400 */
	}

</style>
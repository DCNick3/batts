<script lang="ts">
  import { AutoComplete } from '$lib'
  import { Api } from 'backend'
  import type { UserView } from 'backend'
  import UserSearchItem from './UserSearchItem.svelte'

  export let selectedUser: UserView
  export let placeholder: string = ''
  export let required = false
  export let inputClass = ''
  export let dropdownClassName = ''
  export let localFiltering = false
	export let itemFilterFunction: ((item: UserView, cleanedQuery: string) => boolean) | undefined = undefined

	async function searchFunction(keyword: string) {
		const api = new Api(fetch)
    let options: UserView[] = []
		try {
			const result = await api.searchUsers(keyword)
			if (result.status === 'Success') {
				options = result.payload.top_hits.map(item => item.value )
			} else {
				// TODO: error handling
				console.error(result.payload)
			}
		} catch (error) {
			// TODO: error handling
			console.error(error)
		}
		return options
	}
</script>

<AutoComplete
  class={$$props.class}
  {inputClass}
  {dropdownClassName}
  {searchFunction}
  {placeholder}
  bind:selectedItem={selectedUser}
  {required}
  {localFiltering}
  {itemFilterFunction}
  labelFunction={item => item.name}
>
  <div
    slot="item"
    let:item={item}
    let:label={label}
  >
    <UserSearchItem user={item} />
  </div>
</AutoComplete>
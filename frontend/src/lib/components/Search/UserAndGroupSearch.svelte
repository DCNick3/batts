<script lang="ts">
  import { AutoComplete } from '$lib'
  import { Api } from 'backend'
  import type { ApiResult, SearchResults, GroupView, UserView } from 'backend'
  import UserSearchItem from './UserSearchItem.svelte'

  type Destination
    = { type: 'Group', view: GroupView }
    | { type: 'User', view: UserView }
  export let destination: Destination
  export let placeholder: string = ''
  export let required = false
  export let inputClass = ''

	async function searchFunction(keyword: string) {
		const api = new Api(fetch)
		const promises: [Promise<ApiResult<SearchResults<UserView>>>, Promise<ApiResult<SearchResults<GroupView>>>]
			= [api.searchUsers(keyword), api.searchGroups(keyword)]
		let options: Destination[] = []
		try {
			const [usrRes, grpRes] = await Promise.all(promises)
			if (usrRes.status === 'Success') {
				options = options.concat(usrRes.payload.top_hits.map(item => ({ type: 'User', view: item.value })))
			} else {
				// TODO: error handling
				console.error(usrRes.payload)
			}
			if (grpRes.status === 'Success') {
				options = options.concat(grpRes.payload.top_hits.map(item => ({ type: 'Group', view: item.value })))
			} else {
				// TODO: error handling
				console.error(usrRes.payload)
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
  {searchFunction}
  {placeholder}
  bind:selectedItem={destination}
  {required}
  localFiltering={false}
  labelFunction={(item) => item.type === 'User' ? item.view.name : item.view.title}
>
  <div
    slot="item"
    let:item={item}
    let:label={label}
  >
    {#if item.type === 'Group'}
      {@html label}
    {:else}
      <UserSearchItem user={item.view} />
    {/if}
  </div>
</AutoComplete>
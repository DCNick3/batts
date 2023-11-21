<script lang="ts">
	import type { Writable } from 'svelte/store'
  import type { PageData } from './$types'
  import GroupPage from './GroupView.svelte'
	import { getContext } from 'svelte'
	import { pushApiError, pushError } from '$lib'

  export let data: PageData

  const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
	if (data.error) {
		if (data.error.type === 'Api') {
			pushApiError(errorContext, data.error.error)
		} else {
			pushError(errorContext, data.error.error)
		}
	}
</script>

{#if data.groupInfo === null}
  <!-- TODO: throw 404 page? -->
  <div>Group Not Found</div>
{:else}
  <GroupPage
    group={data.groupInfo.view}
    groupUsers={data.groupInfo.users}
    curUser={data.user}
  />
{/if}

<script lang="ts">
	import { pushApiError, pushError } from '$lib'
  import { UserProfile } from '$lib/components/UserProfile'
  import { getContext } from 'svelte'
	import type { Writable } from 'svelte/store'
	import type { PageData } from './$types'

  export let data: PageData

  const errorContext: Writable<{ title: string, message: string }[]> = getContext('error')
  if (data.error) {
    pushApiError(errorContext, data.error.error)
  }
</script>

<svelte:head>
  <title>{data.userProfile === null ? 'User Profile' : data.userProfile.name}</title>
</svelte:head>

{#if data.userProfile === null}
  <!-- TODO: throw 404 page? -->
  <div>User Not Found</div>
{:else}
  <UserProfile
    user={data.userProfile}
    groups={data.userGroups}
  />
{/if}

<script lang="ts">
  import { UserProfile } from "$lib/components/UserProfile"
	import type { PageData } from "./$types";

  export let data: PageData
</script>

{#if data.userProfile === null}
  <!-- TODO: throw 404 page? -->
  <div>User Not Found</div>
{:else}
  <UserProfile
    user={data.userProfile}
    groups={data.userGroups}
  />
{/if}

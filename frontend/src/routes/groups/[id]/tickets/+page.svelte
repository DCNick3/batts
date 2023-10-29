<script lang="ts">
	import type { PageData } from './$types'

  export let data: PageData
</script>

{#if data.group === null}
  <h1 class="text-2xl font-semibold text-center">An error occured</h1>
{:else}
  <h1 class="text-2xl font-semibold text-center">Tickets of {data.group.title}</h1>
{/if}

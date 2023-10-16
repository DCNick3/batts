<script lang="ts">
  import {
		Button,
	} from 'flowbite-svelte'
</script>

<Button class="text-xl text-black border border-primary-400 p-4 rounded-xl flex justify-start gap-4 bg-transparent hover:bg-transparent">
  <slot />
</Button>
<script lang="ts">
	import { Badge } from 'flowbite-svelte'

  function status2color (status: string) {
		if (status.toLowerCase() === "pending") return "yellow"
		if (status.toLowerCase() === "fixed") return "green"
		if (status.toLowerCase() === "in progress") return "blue"
		if (status.toLowerCase() === "declined") return "red"
		return "primary"
	}

  export let status: string
</script>

<Badge
	class={$$props.class}
	rounded
	color={status2color(status)}
	on:click
>
  {status}
</Badge>

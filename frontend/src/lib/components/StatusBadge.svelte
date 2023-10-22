<script lang="ts">
	import { Badge } from 'flowbite-svelte'

  function status2color (status: string) {
		if (status === "Pending") return "yellow"
		if (status === "Fixed") return "green"
		if (status === "In progress") return "blue"
		if (status === "Declined") return "red"
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
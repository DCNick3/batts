<script lang="ts">
	import { Badge } from 'flowbite-svelte'

  function status2color (status: string) {
		if (status === "Pending") return "yellow"
		if (status === "Fixed") return "green"
		if (status === "In process") return "blue"
		return "primary"
	}

  export let status: string
</script>

<Badge
	class={$$props.class}
	rounded
	color={status2color(status)}
>
  {status}
</Badge>
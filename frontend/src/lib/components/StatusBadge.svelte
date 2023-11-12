<script lang="ts">
	import type { TicketStatus } from 'backend'
	import { Badge } from 'flowbite-svelte'

  function status2color (status: TicketStatus) {
		if (status.toLowerCase() === "pending") return "yellow"
		if (status.toLowerCase() === "fixed") return "green"
		if (status.toLowerCase() === "in progress") return "blue"
		if (status.toLowerCase() === "declined") return "red"
		return "primary"
	}

	type Color = "red" | "yellow" | "green" | "blue" | "primary" | "none" | "indigo" | "purple" | "pink" | "dark" | undefined

  export let status: TicketStatus
	$: [text, color] = (
		status === 'Pending' ? ['Pending', 'yellow'] :
		status === 'Fixed' ? ['Fixed', 'green'] :
		status === 'InProgress' ? ['In Progress', 'blue'] :
		status === 'Declined' ? ['Declined', 'red'] :
		['', undefined] // should never occur
	) as [string, Color]
</script>

<Badge
	class={$$props.class}
	rounded
	color={color}
	on:click
>
  {text}
	<slot />
</Badge>

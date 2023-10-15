<script lang="ts">
	import { TimelineItem } from '$lib/components/Timeline';

  import type { TicketTimelineItem } from 'backend'

  export let users: Map<string, string>
  export let item: TicketTimelineItem
  $: content = item.content

  // https://stackoverflow.com/questions/3426404/create-a-hexadecimal-colour-based-on-a-string-with-javascript
  const stringToColour = (str: string) => {
    let hash = 0;
    str.split('').forEach(char => {
      hash = char.charCodeAt(0) + ((hash << 5) - hash)
    })
    let colour = '#'
    for (let i = 0; i < 3; i++) {
      const value = (hash >> (i * 8)) & 0xff
      colour += value.toString(16).padStart(2, '0')
    }
    return colour
  }
</script>

{#if (content.type === 'StatusChange')}

<TimelineItem
  title={`Status changed from ${content.old} to ${content.new}.`}
  date={item.date}
>
</TimelineItem>

{:else}

<TimelineItem>
  <svelte:fragment slot="icon">
    <div
      class={`absolute w-10 h-10 rounded-full -left-5`}
      style:background-color={stringToColour(content.from)}
    />
  </svelte:fragment>
  <div class="px-5 py-2 ml-4 border rounded-lg">
    <div class="flex justify-between">
      <span class="text-lg font-semibold text-gray-900 mb-2">
        {users.get(content.from) || "Unknown user"}
      </span>
      <time class="font-normal text-sm text-gray-400 ml-2">
        {item.date}
      </time>
    </div>

    <div>
      {content.text}
    </div>
  </div>
</TimelineItem>

{/if}

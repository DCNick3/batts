<script lang="ts">
	import { TimelineItem } from '$lib/components/Timeline';

  import type { Message } from '$lib/mocks/database'
  import type { TicketTimelineItem } from '$backend/bindings/TicketTimelineItem'

  export let item: TicketTimelineItem
  $: content = item.content
  export let sender: string
  export let receiver: string
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
    <!-- <div class={"absolute w-10 h-10 rounded-full -left-5 " + (item.is_sender ? "bg-teal-300" : "bg-amber-600")} /> -->
    <div class="absolute w-10 h-10 rounded-full -left-5 bg-teal-300" />
  </svelte:fragment>
  <div class="px-5 py-2 ml-4 border rounded-lg">
    <div class="flex justify-between">
      <span class="text-lg font-semibold text-gray-900 mb-2">
        {sender}
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

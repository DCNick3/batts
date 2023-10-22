<script lang="ts">
	import { TimelineItem } from '$lib/components/Timeline'

  import type { TicketTimelineItem } from 'backend'
  import Time from '$lib/components/Time.svelte'
  import Avatar from '$lib/components/Avatar.svelte'

  export let users: Map<string, string>
  export let item: TicketTimelineItem
  $: content = item.content
</script>

{#if (content.type === 'StatusChange')}

<TimelineItem
  title={`Status changed from ${content.old} to ${content.new}.`}
  date={item.date}
>
</TimelineItem>

{:else if (content.type === 'Message')}

<TimelineItem>
  <svelte:fragment slot="icon">
    <Avatar
      class="absolute -left-5 max-sm:hidden"
      str={content.from}
    />
  </svelte:fragment>
  <div class="px-5 py-2 sm:ml-4 border rounded-lg">
    <div class="flex justify-between items-center mb-1">
      <span class="text-lg font-semibold text-gray-900 mb-1">
        {users.get(content.from) || "Unknown user"}
      </span>
      <Time
        time={item.date}
      />
    </div>

    <div class="leading-5">
      {content.text}
    </div>
  </div>
</TimelineItem>

{:else}
  <TimelineItem
  title={`Assignee changed from ${content.old ? users.get(content.old) : 'no-one'} to ${content.new ? users.get(content.new) : 'no-one'}.`}
  date={item.date}
  >
  </TimelineItem>

{/if}

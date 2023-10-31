<script lang="ts">
	import { TimelineItem } from '$lib/components/Timeline'

  import type { TicketTimelineItem, UserId, UserProfileView } from 'backend'
  import Time from '$lib/components/Time.svelte'
  import Avatar from '$lib/components/Avatar.svelte'

  export let users: Record<UserId, UserProfileView>
  export let item: TicketTimelineItem
  $: content = item.content
  $: getUsr = (id: UserId) => {
    const usr = users[id]
    if (usr) {
      return usr.name
    } else {
      return null
    }
  }
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
        {getUsr(content.from) || "Unknown user"}
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
  title={`Assignee changed from ${content.old ? getUsr(content.old) : 'no-one'} to ${content.new ? getUsr(content.new) : 'no-one'}.`}
  date={item.date}
  >
  </TimelineItem>

{/if}

<script lang="ts">
	import { TimelineItem } from '$lib/components/Timeline'

  import type { TicketTimelineItem, UserId, UserProfileView } from 'backend'
  import Time from '$lib/components/Time.svelte'
  import Avatar from '$lib/components/Avatar.svelte'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import A from '$lib/components/A.svelte'

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
  date={item.date}
>
  <h3 class="ml-4 text-sm font-medium text-gray-700 dark:text-white">
    Status changed from
    <StatusBadge status={content.old} />
    to
    <StatusBadge status={content.new} />
  </h3>
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
  date={item.date}
  >
    <h3 class="ml-4 text-sm font-medium text-gray-700 dark:text-white">
      Assignee changed from
      {#if content.old !== null && getUsr(content.old) !== null}
        <A
          class="visited:text-gray-900 text-gray-900"
          href={`/users/${content.old}`}
        >
          {getUsr(content.old)}
        </A>
      {:else}
        <span class="underline">no-one</span>
      {/if}
      to
      {#if content.new !== null && getUsr(content.new) !== null}
        <A
          class="visited:text-gray-900 text-gray-900"
          href={`/users/${content.new}`}
        >
          {getUsr(content.new)}
        </A>
      {:else}
        <span class="underline">no-one</span>
      {/if}
    </h3>
  </TimelineItem>

{/if}

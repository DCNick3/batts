<script lang="ts">
	import { TimelineItem } from '$lib/components/Timeline';

  import type { Message } from '$lib/server/database'

  export let message: Message
  export let sender: string
  export let receiver: string
</script>

{#if (message.type === 'notification')}

<TimelineItem
  title={message.text}
  date={message.date}
>
  <!-- <div class="px-5 py-2 ml-4 border rounded-lg">
    <div class="flex justify-between">
      <span class="text-lg font-semibold text-gray-900 mb-2">
        {message.is_sender ? sender : receiver}
      </span>
      <time class="font-normal text-sm text-gray-400 ml-2">
        {message.date}
      </time>
    </div>
    
    <div>
      {message.text}
    </div>
  </div> -->
</TimelineItem>

{:else}

<TimelineItem>
  <svelte:fragment slot="icon">
    <div class={"absolute w-10 h-10 rounded-full -left-5 " + (message.is_sender ? "bg-teal-300" : "bg-amber-600")} />
  </svelte:fragment>
  <div class="px-5 py-2 ml-4 border rounded-lg">
    <div class="flex justify-between">
      <span class="text-lg font-semibold text-gray-900 mb-2">
        {message.is_sender ? sender : receiver}
      </span>
      <time class="font-normal text-sm text-gray-400 ml-2">
        {message.date}
      </time>
    </div>
    
    <div>
      {message.text}
    </div>
  </div>
</TimelineItem>

{/if}

<script lang="ts">
  import { Timeline, TimelineItem } from '$lib/components/Timeline'
  import Ticket from './Ticket.svelte'
  export let data
</script>

<div class="flex flex-col items-center gap-8">
  <h1 class="text-2xl font-semibold">{data.topic}</h1>
  <!-- <div class="flex flex-col items-center">
    <span>To: {data.receiver}</span>
    <span>From: {data.sender}</span>  
  </div> -->

  <Timeline>
    {#each data.messages as message}
      <Ticket message={message} sender={data.sender} receiver={data.receiver} />
    {/each}
  </Timeline>
</div>

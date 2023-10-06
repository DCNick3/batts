<script>
  import { Timeline, TimelineItem } from 'flowbite-svelte';
  export let data
</script>

<Timeline order="vertical">
  {#each data.requests as ticket}
    <TimelineItem title={ticket.topic}>
      <p><strong>To:</strong> {ticket.receiver}</p>
      <p><strong>Status:</strong> {ticket.status}</p>
    </TimelineItem>
  {/each}
</Timeline>
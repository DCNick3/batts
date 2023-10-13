<script lang="ts">
  import { Timeline } from '$lib/components/Timeline'
  import StatusBadge from '$lib/components/StatusBadge.svelte'
  import Ticket from './Ticket.svelte'
  import { Button, Textarea } from 'flowbite-svelte'
  import type { PageData } from './$types'
  export let data: PageData
</script>

<div class="mx-10 grid grid-cols-[3fr_1fr] gap-y-10 gap-x-14">
  <h1 class="text-2xl font-semibold text-center">{data.title}</h1>

  <div></div>

  <div class="flex flex-col items-center gap-4 basis-3/4">  
    <Timeline class="w-full">
      {#each data.timeline as item}
        <Ticket item={item} />
      {/each}
    </Timeline>
    <Textarea
      class="mt-2 resize-none"
      name="message"
      rows=4
      placeholder="Write a message"
    />
	  <Button class="w-full" type="submit">Send message</Button>
</div>

  <div class="flex flex-col gap-6 basis-1/4">
    <div>
      <div class="font-semibold text-zinc-600">Submitted To</div>
      <div class="font-normal text-sm">TODO</div>  
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Requested By</div>
      <div class="font-normal text-sm">{data.owner}</div>
    </div>

    <div>
      <div class="font-semibold text-zinc-600">Status</div>
      <StatusBadge status={data.status} />
    </div>
  </div>
</div>

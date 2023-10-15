<script lang="ts">
  import { twMerge } from 'tailwind-merge'

  export let time: string

  const date = new Date(time)
  const now = new Date()

  let display = ''

  const stringify = n => n < 10 ? `0${n}` : `${n}`

  if (now.getDay() === date.getDay()) {
    let hours = date.getHours()
    let minutes = date.getMinutes()
    display = `${stringify(hours)}:${stringify(minutes)}`
  } else {
    display = date.toString().split(' ').slice(1, 3).reverse().join(' ')
  }
</script>

<time class={twMerge("font-normal text-sm text-gray-400", $$props.class)}>
  {display}
</time>

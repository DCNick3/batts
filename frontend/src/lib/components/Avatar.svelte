<script lang="ts">
  import { twMerge } from 'tailwind-merge'
  export let str: string
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

<div
  class={twMerge('w-10 h-10 rounded-full', $$props.class)}
  style:background-color={stringToColour(str)}
/>

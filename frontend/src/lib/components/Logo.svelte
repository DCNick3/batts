<script>
	import { twMerge } from 'tailwind-merge';
	import logo from '$lib/assets/batts_logo_512.png'

  $: logoClass = twMerge("h-20 w-20 self-center", $$props.class)
</script>

<img src={logo} alt="A green bat" class={logoClass} />

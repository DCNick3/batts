<script>
	import '../app.postcss'
	import '../style.css'
	import Logo from '$lib/components/Logo.svelte'
</script>

<div class="page">
	<aside class="flex flex-col items-center w-64 bg-slate-50 p-4">
		<a href="/" class="block w-fit">
			<Logo />
		</a>
	</aside>

	<div class="flex flex-col w-full p-10 gap-6">
		<slot />
	</div>
</div>

<style>
	.page {
		display: flex;
		flex: 1 1 100%;
		height: 100vh;
	}
</style>

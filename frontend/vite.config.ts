import { fileURLToPath, URL } from "url";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	resolve: {
		alias: [
			{ find: "backend", replacement: fileURLToPath(new URL('../backend', import.meta.url)) }
		]
	}
});

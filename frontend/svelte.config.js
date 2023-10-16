import adapter
	from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: [vitePreprocess({})],

	kit: {
		adapter: adapter(),
		csp: {
			directives: {
				"script-src": ['https://telegram.org', 'unsafe-eval'],
				"frame-src": ['https://t.me', 'https://oauth.telegram.org/'],
				"frame-ancestors": ['https://oauth.telegram.org/', 'http://localtest.me']
			}
		}
	}
};

export default config;

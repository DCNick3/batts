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
		paths: {
			// Work around a race condition in rrweb while capturing a session replay
			// if the user navigates away from the page before the session is serialized, the css href will be resolved relative to an incorrect base.
			// This will hopefully force svelte to use an absolute path for the css href
			assets: "https://batts.tatar"
		}
	},
};

export default config;

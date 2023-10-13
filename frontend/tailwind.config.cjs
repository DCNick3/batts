/** @type {import('tailwindcss').Config}*/
const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'../node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
	],

	theme: {
		extend: {
			colors: {
				// flowbite-svelte
				// primary: {
          // 50: '#FFF5F2',
          // 100: '#FFF1EE',
          // 200: '#FFE4DE',
          // 300: '#FFD5CC',
          // 400: '#FFBCAD',
          // 500: '#FE795D',
          // 600: '#EF562F',
          // 700: '#EB4F27',
          // 800: '#CC4522',
          // 900: '#A5371B'
        // },

				primary: {
					50: '#f0fcf6', 
					100: '#e1faed', 
					200: '#b6f2d0', 
					300: '#8debaf', 
					400: '#43d964', 
					500: '#02c913', 
					600: '#02b511', 
					700: '#02960b', 
					800: '#017809', 
					900: '#005c06', 
					950: '#003b03'
				}
			}
		}
	},

	plugins: [require('flowbite/plugin')]
};

module.exports = config;

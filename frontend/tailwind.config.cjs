/** @type {import('tailwindcss').Config}*/
const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'../node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
	],

	theme: {
		extend: {
			colors: {
				// primary: {
				// 	50: '#f2fcf9', 
				// 	100: '#e6faf3', 
				// 	200: '#bdf0db', 
				// 	300: '#9ae6c2', 
				// 	400: '#59d48e', 
				// 	500: '#20bf55', 
				// 	600: '#1aad49', 
				// 	700: '#138f38', 
				// 	800: '#0b7327', 
				// 	900: '#07571b', 
				// 	950: '#03380e'
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

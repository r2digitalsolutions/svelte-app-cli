/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@r2digisolutions/svelte-ui/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite/**/*.{html,js,svelte,ts}",
  ],
  theme: {
    extend: {},
  },
  darkMode: "class",
  plugins: [],
};

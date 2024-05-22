/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  purge: [ "./src/**/*.svelte",
    // may also want to include HTML files
    "./src/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
      require('daisyui'),
  ],
  daisyui: {
    darkTheme: "cupcake",
  },
}

/** @type {import('tailwindcss').Config} */
module.exports = {
  // content: ["./src/**/*.{html,js}"],
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  theme: {
    extend: {},
  },
  corePlugins: { //https://miyauchi.dev/posts/lib-vite-tailwindcss/
    preflight: false,
  },
  // prefix: 'ui-', //https://miyauchi.dev/posts/lib-vite-tailwindcss/
  plugins: [], // 
};

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      listStyleImage: {
        moon: 'url(./assets/image/moon-solid-24.png)'
      }
    },
  },
  plugins: [],
}


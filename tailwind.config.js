/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './templates/**/*.html',
  ],
  theme: {
    extend: {
      colors: {
        current: 'currentColor',
      }
    },
  },
  plugins: [
    require("@tailwindcss/forms"),
  ],
}
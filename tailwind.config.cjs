const path = require('path')

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    path.resolve(__dirname, 'src-web/**/*.{js,ts,jsx,tsx}'),
    path.resolve(__dirname, 'src-web/index.html'),
  ],
  theme: {
    extend: {
      colors: ({ colors }) => ({
        primary: colors.blue,
      }),
    },
  },
  plugins: [require('@tailwindcss/forms')],
}

const colors = require('tailwindcss/colors')
const { fontFamily } = require('tailwindcss/defaultTheme')
const withTwistail = require('@twistail/core/config')

/** @type {import('tailwindcss').Config} */
const tailwindConfig = {
  content: ['src/**/*!(*.stories|*.spec).{ts,tsx}'],
  darkMode: ['class'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', ...fontFamily.sans],
        mono: ['JetBrains Mono', ...fontFamily.mono],
      },
      colors: {
        gray: colors.zinc,
        primary: colors.indigo,
        secondary: colors.rose,
      },
    },
  },
  plugins: [],
}

module.exports = withTwistail(tailwindConfig)

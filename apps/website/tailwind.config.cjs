const colors = require('tailwindcss/colors')
const defaultTheme = require('tailwindcss/defaultTheme')
const withTwistail = require('@twistail/core/config')

/** @type {import('tailwindcss').Config} */
const tailwindConfig = {
  content: ['src/**/*!(*.stories|*.spec).{ts,tsx}'],
  darkMode: ['class'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', ...defaultTheme.fontFamily.sans],
        mono: ['var(--jetbrains-mono-font)', ...defaultTheme.fontFamily.mono],
      },
      colors: {
        black: '#121314',
        gray: colors.gray,
        primary: colors.sky,
        secondary: colors.violet,
      },
    },
  },
  plugins: [],
}

module.exports = withTwistail(tailwindConfig)
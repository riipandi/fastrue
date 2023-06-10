import type { Config } from 'tailwindcss'
import defaultTheme from 'tailwindcss/defaultTheme'

export default {
  content: ['websrc/**/*.{js,ts,jsx,tsx}', 'websrc/index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: [...defaultTheme.fontFamily.sans],
        mono: [...defaultTheme.fontFamily.mono],
      },
      colors: ({ colors }) => ({
        primary: colors.blue,
      }),
    },
    debugScreens: {
      position: ['bottom', 'left'],
    },
  },
  plugins: [
    require('@tailwindcss/aspect-ratio'),
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
    require('tailwind-scrollbar-hide'),
    require('tailwindcss-debug-screens'),
  ],
} satisfies Config

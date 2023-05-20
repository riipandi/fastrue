import type { Config } from 'tailwindcss'
import defaultTheme from 'tailwindcss/defaultTheme'

export default {
  content: ['websrc/**/*.{js,ts,jsx,tsx}', 'websrc/index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', ...defaultTheme.fontFamily.sans],
        mono: ['JetBrains Mono', ...defaultTheme.fontFamily.mono],
      },
      colors: ({ colors }) => ({
        primary: colors.blue,
      }),
    },
  },
  plugins: [require('@tailwindcss/forms')],
} satisfies Config

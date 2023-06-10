import { tv } from 'tailwind-variants'

export const button = tv({
  base: 'font-medium bg-blue-500 hover:bg-opacity-80 text-white rounded-lg active:opacity-80 focus:outline-none',
  variants: {
    color: {
      primary: 'bg-blue-500 text-white',
      secondary: 'bg-purple-500 text-white',
    },
    size: {
      sm: 'text-sm',
      md: 'text-base',
      lg: 'px-6 py-3 text-lg',
    },
  },
  compoundVariants: [
    {
      size: ['sm', 'md'],
      class: 'px-4 py-2',
    },
  ],
  defaultVariants: {
    size: 'md',
    color: 'primary',
  },
})

import React from 'react'
import { Button as AriaButton } from '@ariakit/react'
import { twMerge } from 'tailwind-merge'

import { PolymorphicComponentPropWithRef, PolymorphicRef } from '@/utils/polymorphic'

// import { type VariantProps } from 'tailwind-variants'
import { button } from './styles'

/**
 * This is the updated component props using PolymorphicComponentPropWithRef
 */
type ButtonProps<T extends React.ElementType> = PolymorphicComponentPropWithRef<
  T,
  {
    variant?: 'primary' | 'secondary'
    size?: 'sm' | 'md' | 'lg'
    className?: string
    loading?: boolean
  }
>

/**
 * This is the type used in the type annotation for the component
 */
type ButtonComponentType = <T extends React.ElementType = 'button'>(
  props: ButtonProps<T>
) => React.ReactElement | null

const ButtonComponent = <T extends React.ElementType = 'button'>(
  { as, variant: color, size, loading, className, children, disabled, ...rest }: ButtonProps<T>,
  ref?: PolymorphicRef<T>
) => {
  const Component = as || AriaButton
  return (
    <Component
      className={button({ size, color, className: twMerge(className) })}
      disabled={disabled || loading}
      ref={ref}
      {...rest}
    >
      {children}
    </Component>
  )
}

export const Button: ButtonComponentType = React.forwardRef(ButtonComponent)

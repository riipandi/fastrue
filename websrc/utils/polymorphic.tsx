export type AsProp<T extends React.ElementType> = {
  as?: T
}

export type PropsToOmit<T extends React.ElementType, P> = keyof (AsProp<T> & P)

// This is the first reusable type utility we built
export type PolymorphicComponentProp<
  T extends React.ElementType,
  Props = {}
> = React.PropsWithChildren<Props & AsProp<T>> &
  Omit<React.ComponentPropsWithoutRef<T>, PropsToOmit<T, Props>>

// This is a new type utitlity with ref!
export type PolymorphicComponentPropWithRef<
  T extends React.ElementType,
  Props = {}
> = PolymorphicComponentProp<T, Props> & { ref?: PolymorphicRef<T> }

// This is the type for the "ref" only
export type PolymorphicRef<T extends React.ElementType> = React.ComponentPropsWithRef<T>['ref']

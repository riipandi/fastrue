/**
 * This file is required to use MDX in `app` directory. This file allows you
 * to provide custom React components to be used in MDX files. You can import
 * and use any React component you want, including components from other
 * libraries.
 *
 * @reference: https://contentlayer.dev/docs/sources/files/mdx-d747e46d
 */

import Link from 'next/link'
import type { MDXComponents } from 'mdx/types'

// Define your custom MDX components.
// Allows customizing built-in components, e.g. to add styling.
export const mdxComponents: MDXComponents = {
  h1: ({ children }) => <h1 className='mt-5 text-4xl font-bold'>{children}</h1>,
  h2: ({ children }) => <h2 className='mt-5 text-2xl font-bold'>{children}</h2>,
  h3: ({ children }) => <h3 className='mt-5 text-xl font-bold'>{children}</h3>,
  h4: ({ children }) => <h4 className='text-baes mt-5 font-semibold'>{children}</h4>,
  h5: ({ children }) => <h5 className='mt-6 text-sm font-semibold'>{children}</h5>,
  p: ({ children }) => <p className='mt-2 font-normal'>{children}</p>,
  a: ({ children, href }) => {
    return href !== undefined ? (
      <Link href={href} className='text-blue-600 hover:underline'>
        {children}
      </Link>
    ) : (
      <span className='text-blue-500 hover:underline'>{children}</span>
    )
  },
}

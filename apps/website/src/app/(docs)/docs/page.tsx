import type { Metadata } from 'next'
import Image from 'next/image'
import Link from 'next/link'

import { ExternalLink } from '@/components/external-link'

import VercelLogo from '~/images/vercel.svg'

export const metadata: Metadata = {
  title: {
    absolute: 'Fastrue Documentation',
  },
}

export default async function Page() {
  return (
    <>
      <main className='mx-auto flex w-full max-w-4xl grow flex-col justify-center px-4 sm:px-6 lg:px-8'>
        <div className='py-16'>
          <div className='text-center'>
            <h1 className='mt-4 text-2xl font-bold tracking-tight text-gray-900 dark:invert sm:text-3xl lg:text-5xl'>
              Fastrue documentation
            </h1>
            <div className='mt-8 sm:mt-12'>
              <Link
                href='/'
                className='inline-flex items-center rounded-lg border border-gray-200 bg-gray-900 px-6 py-3 text-center text-sm font-medium text-white hover:bg-gray-700 hover:text-gray-100 focus:outline-none focus:ring-4 focus:ring-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-700 dark:focus:ring-gray-600'
              >
                <span className='i-heroicons-chevron-double-left -ml-1 mr-1 h-4 w-4' />
                Back to homepage
              </Link>
            </div>
          </div>
        </div>
      </main>
      <footer className='mx-auto w-full max-w-7xl shrink-0 px-4 sm:px-6 lg:px-8'>
        <p className='flex justify-center'>
          <span className='mr-1 text-gray-600 dark:text-gray-200'>Brought to you by</span>
          <ExternalLink
            href='https://twitter.com/riipandi'
            className='group inline-flex items-center space-x-1 text-primary-800 hover:text-red-600 dark:text-primary-500 dark:hover:text-primary-700'
          >
            <span>Aris Ripandi</span>
            <span className='i-heroicons-arrow-top-right-on-square dark:invert' />
          </ExternalLink>
        </p>
      </footer>
    </>
  )
}

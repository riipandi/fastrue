import type { Metadata } from 'next'
import Link from 'next/link'
import { getMDXComponent } from 'next-contentlayer/hooks'
import { allDocs, Doc } from 'contentlayer/generated'
import { compareDesc, format, parseISO } from 'date-fns'

import { ExternalLink } from '@/components/external-link'

export const metadata: Metadata = {
  title: {
    absolute: 'Fastrue Documentation',
  },
}

function DocCard(doc: Doc) {
  const Content = getMDXComponent(doc.body.code)

  return (
    <div className='mb-8'>
      <h2 className='text-xl'>
        <Link href={doc.url} className='text-blue-700 hover:text-blue-900' legacyBehavior>
          {doc.title}
        </Link>
      </h2>
      <time dateTime={doc.date} className='block mb-2 text-xs text-gray-600'>
        {format(parseISO(doc.date), 'LLLL d, yyyy')}
      </time>
      <div className='text-sm'>
        <Content />
      </div>
    </div>
  )
}

export default async function Page() {
  const docs = allDocs.sort((a, b) => compareDesc(new Date(a.date), new Date(b.date)))

  return (
    <>
      <main className='mx-auto flex w-full max-w-4xl grow flex-col justify-center px-4 sm:px-6 lg:px-8'>
        <div className='py-16'>
          <h1 className='mt-4 text-2xl font-bold tracking-tight text-gray-900 dark:invert sm:text-3xl lg:text-5xl'>
            Fastrue documentation
          </h1>
          <div className='mt-8 sm:mt-12'>
            {docs.map((item, idx) => (
              <DocCard key={idx} {...item} />
            ))}
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

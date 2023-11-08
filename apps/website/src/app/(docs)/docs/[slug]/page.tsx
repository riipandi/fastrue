import { notFound } from 'next/navigation'
import { getMDXComponent } from 'next-contentlayer/hooks'
import { allDocs } from 'contentlayer/generated'
import { format, parseISO } from 'date-fns'

import { mdxComponents } from '@/components/mdx-components'

export const generateStaticParams = async () =>
  allDocs.map((post) => ({ slug: post._raw.flattenedPath }))

interface PageParams {
  params: { slug: string }
}

export const generateMetadata = ({ params }: PageParams) => {
  const post = allDocs.find((post) => post._raw.flattenedPath === params.slug)
  return !post
    ? { title: { absolute: 'Fastrue Documentation' } }
    : { title: { absolute: post.title } }
}

export default function Page({ params }: PageParams) {
  const post = allDocs.find((post) => post._raw.flattenedPath === params.slug)

  if (!post) {
    return notFound()
  }

  const Content = getMDXComponent(post.body.code)

  return (
    <>
      <div className='flex-1 max-w-3xl mx-auto w-full'>
        {' '}
        <article className='py-8 mx-auto max-w-xl'>
          <div className='mb-6'>
            <time dateTime={post.date} className='mb-1 text-xs text-gray-600'>
              {format(parseISO(post.date), 'LLLL d, yyyy')}
            </time>
            <h1 className='text-2xl font-semibold'>{post.title}</h1>
          </div>
          <article className='prose prose-base prose-blue'>
            <Content components={mdxComponents} />
          </article>
        </article>
      </div>
      {/* <Aside items={toc} /> */}
    </>
  )
}

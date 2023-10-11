import { notFound } from 'next/navigation'
import { getMDXComponent } from 'next-contentlayer/hooks'
import { allDocs } from 'contentlayer/generated'
import { format, parseISO } from 'date-fns'

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
    <article className='py-8 mx-auto max-w-xl'>
      <div className='mb-8'>
        <time dateTime={post.date} className='mb-1 text-xs text-gray-600'>
          {format(parseISO(post.date), 'LLLL d, yyyy')}
        </time>
        <h1>{post.title}</h1>
      </div>
      <Content />
    </article>
  )
}

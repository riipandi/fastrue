'use client'

import { cn } from '@twistail/core/utils'

export default function Aside({ items }: { items: Array<[]> }) {
  return (
    <div className='hidden lg:block sticky top-[55px] h-max'>
      <nav className='w-64 px-5'>
        {items.length > 0 && (
          <div>
            <h3 className='font-semibold text-sm mt-10 mb-2'>On This Page</h3>
            <TableOfContents outline={items} />
          </div>
        )}
      </nav>
    </div>
  )
}

const getChildrenText = ({ contents }: { contents: any }) => {
  return contents.map((node: any) => (typeof node === 'string' ? node : node.text || '')).join('')
}

const TableOfContents = ({ subheading, outline }: any) => (
  <ol className={cn(' text-sm', subheading && 'mt-1 ml-6')}>
    {outline.map((heading: any) => (
      <li key={heading._key}>
        <a
          className='inline-flex py-1 text-slate-600 hover:text-violet-500 focus:text-violet-500'
          href={'#' + heading.slug}
        >
          {getChildrenText(heading)}
        </a>
        {heading.subheadings.length > 0 && (
          <TableOfContents outline={heading.subheadings} subheading={true} />
        )}
      </li>
    ))}
  </ol>
)

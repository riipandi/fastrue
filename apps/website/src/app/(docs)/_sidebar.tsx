'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { cn } from '@twistail/core/utils'

import { ExternalLink } from '@/components/external-link'

interface SidebarProps {
  items: Array<{
    title: string
    url: string
  }>
}

export default function Sidebar({ items }: SidebarProps) {
  const pathname = usePathname()

  return (
    <div className='sticky h-[calc(100vh-55px)] border-r  border-gray-200 top-[55px] hidden md:block'>
      <div
        className={cn(
          'lg:w-64 pb-5 px-5 overflow-y-auto'
          // !props.mobile && 'h-[calc(100%-53px)]'
        )}
      >
        <ul className='py-4'>
          {items.map((item, idx) => (
            <li key={idx}>
              <Link
                href={item.url}
                className={cn(
                  'inline-flex py-1 text-sm  hover:text-violet-500 focus:text-violet-500',
                  pathname === item.url ? 'text-violet-500' : 'text-slate-600'
                )}
              >
                {item.title}
              </Link>
            </li>
            // <li key={index} className='mt-5'>
            //   <span className=' font-semibold'> {item.title}</span>
            //   <ul className='mt-2 mx-4'>
            //     {item.items.map((doc: any, docindex: number) => (
            //       <li key={docindex}>
            //         <Link href={item.url}>
            //           <a
            //             // onClick={props.closeModal}
            //             className={cn(
            //               'inline-flex py-1 text-sm  hover:text-violet-500 focus:text-violet-500',
            //               doc.slug.current === props.active ? 'text-violet-500' : 'text-slate-600'
            //             )}
            //           >
            //             {' '}
            //             {doc.title}
            //           </a>
            //         </Link>
            //       </li>
            //     ))}
            //   </ul>
            // </li>
          ))}
        </ul>
      </div>
      <div className='border-t border-gray-200 p-4 absolute bottom-0 w-full'>
        <p className='text-sm text-slate-500'>
          Made by{' '}
          <ExternalLink
            href='https://twitter.com/riipandi'
            className='text-violet-500 hover:text-violet-600 inline-flex items-center gap-1'
          >
            <span>Aris Ripandi</span>
            <span className='i-lucide-external-link w-4 h-4' />
          </ExternalLink>
        </p>
      </div>
    </div>
  )
}

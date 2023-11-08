'use client'

import Image from 'next/image'
import Link from 'next/link'
import { cn } from '@twistail/core/utils'

import { ExternalLink } from '@/components/external-link'

// import MobileMenu from './menu'
import NextjsLogo from '~/images/next.svg'

export default function Navbar() {
  return (
    <header
      className={cn(
        'sticky top-0 z-10 border-b bg-white',
        ' flex items-center justify-between px-5 py-3 '
      )}
    >
      <div className='flex items-center gap-2'>
        {/* <div className='md:hidden flex items-center'>
          <MobileMenu sidebar={props.sidebar} active={props.active} />
        </div> */}
        <Link href='/docs' className='flex'>
          <Image src={NextjsLogo} className='h-4 w-auto' alt='Brand Logo' />
        </Link>
      </div>
      <nav className='flex items-center justify-center gap-5'>
        <Link
          href='/'
          className='hidden sm:inline-flex items-center gap-1 text-sm border-b text-slate-900 hover:text-violet-500 hover:border-violet-200 focus-visible:bg-violet-100 focus-visible:border-violet-100 focus-visible:outline-4 focus-visible:outline-violet-100'
        >
          Homepage
        </Link>
        <ExternalLink
          href='https://github.com/riipandi/fastrue/releases'
          className='hidden sm:inline-flex items-center gap-1 text-sm border-b text-slate-900 hover:text-violet-500 hover:border-violet-200 focus-visible:bg-violet-100 focus-visible:border-violet-100 focus-visible:outline-4 focus-visible:outline-violet-100'
        >
          <span>Download</span>
          <span className='i-lucide-external-link w-3 h-3' />
        </ExternalLink>
        <ExternalLink href='https://github.com/riipandi/fastrue'>
          <span className='i-lucide-github w-5 h-5' />
        </ExternalLink>
        <button type='button' className=''>
          <span className='i-lucide-sun-dim w-5 h-5' />
        </button>
      </nav>
    </header>
  )
}

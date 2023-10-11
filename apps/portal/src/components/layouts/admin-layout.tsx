import { Outlet, Link, useLocation } from 'react-router-dom'

import {
  Button,
  Menu,
  MenuButton,
  MenuButtonArrow,
  MenuItem,
  MenuProvider,
  MenuSeparator,
} from '@ariakit/react'
import { cn } from '@twistail/core/utils'

import reactLogo from '@/assets/react.svg'
import viteLogo from '/vite.svg'

const navlinks = [
  {
    label: 'Overview',
    href: '/admin',
  },
  {
    label: 'Manage Users',
    href: '/admin/manage-users',
  },
  {
    label: 'Activity Logs',
    href: '/admin/activity-logs',
  },
  {
    label: 'Settings',
    href: '/admin/settings',
  },
]

export default function AdminLayout() {
  const location = useLocation()
  return (
    <div className='bg-gray-100 dark:bg-gray-950 w-full min-h-screen h-full'>
      <nav className='bg-gray-800'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <div className='flex h-16 justify-between'>
            <div className='flex'>
              <div className='-ml-2 mr-2 flex items-center md:hidden'>
                {/* Mobile menu button */}
                <Button
                  type='button'
                  className='relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white'
                  aria-controls='mobile-menu'
                  aria-expanded='false'
                >
                  <span className='absolute -inset-0.5' />
                  <span className='sr-only'>Open main menu</span>
                  <svg
                    className={cn('block', 'h-6 w-6')}
                    fill='none'
                    viewBox='0 0 24 24'
                    strokeWidth='1.5'
                    stroke='currentColor'
                    aria-hidden='true'
                  >
                    <path
                      strokeLinecap='round'
                      strokeLinejoin='round'
                      d='M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5'
                    />
                  </svg>
                  {/*
        Icon when menu is open.

        Menu open: "block", Menu closed: "hidden"
      */}
                  <svg
                    className='hidden h-6 w-6'
                    fill='none'
                    viewBox='0 0 24 24'
                    strokeWidth='1.5'
                    stroke='currentColor'
                    aria-hidden='true'
                  >
                    <path strokeLinecap='round' strokeLinejoin='round' d='M6 18L18 6M6 6l12 12' />
                  </svg>
                </Button>
              </div>
              <div className='flex flex-shrink-0 items-center'>
                <img src={viteLogo} className='h-6 w-auto' alt='Fastrue' />
              </div>
              <div className='hidden md:ml-6 md:flex md:items-center md:space-x-4'>
                {navlinks.map((item, idx) => (
                  <Link
                    key={idx}
                    to={item.href}
                    className={cn(
                      location.pathname === item.href
                        ? 'bg-gray-900 text-white'
                        : 'text-gray-300 hover:bg-gray-700 hover:text-white',
                      'rounded-md px-3 py-2 text-sm font-medium'
                    )}
                    aria-current='page'
                  >
                    {item.label}
                  </Link>
                ))}
              </div>
            </div>
            <div className='flex items-center'>
              <div className='flex-shrink-0'>
                <Button
                  type='button'
                  className='relative inline-flex items-center gap-x-1.5 rounded-md bg-primary-500 px-3 py-2 text-sm font-medium text-white shadow-sm hover:bg-primary-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-500'
                >
                  <span className='i-lucide-user-plus-2 -ml-0.5 h-5 w-5' />
                  <span>New User</span>
                </Button>
              </div>
              <div className='hidden md:ml-4 md:flex md:flex-shrink-0 md:items-center'>
                <Button
                  type='button'
                  className='relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800'
                >
                  <span className='absolute -inset-1.5' />
                  <span className='sr-only'>View notifications</span>
                  <span className='i-lucide-bell-dot h-6 w-6' />
                </Button>
                {/* Profile dropdown */}
                <div className='relative ml-3'>
                  <MenuProvider>
                    <MenuButton className='relative flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800'>
                      <span className='absolute -inset-1.5' />
                      <span className='sr-only'>Open user menu</span>
                      <img className='h-8 w-8 rounded-full' src={reactLogo} alt='User avatar' />
                      <MenuButtonArrow className='sr-only' />
                    </MenuButton>
                    <Menu
                      gutter={8}
                      className={cn(
                        'absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none'
                      )}
                    >
                      <MenuItem
                        as={Link}
                        to='/account'
                        className='block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100'
                      >
                        User Profile
                      </MenuItem>
                      <MenuItem
                        as={Link}
                        to='#'
                        className='block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100'
                      >
                        Preferences
                      </MenuItem>
                      <MenuSeparator className='separator' />
                      <MenuItem
                        as={Link}
                        to='/login'
                        className='block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100'
                      >
                        Log out
                      </MenuItem>
                    </Menu>
                  </MenuProvider>
                </div>
              </div>
            </div>
          </div>
        </div>
        {/* Mobile menu, show/hide based on menu state. */}
        <div className='md:hidden' id='mobile-menu'>
          <div className='space-y-1 px-2 pb-3 pt-2 sm:px-3'>
            {navlinks.map((item, idx) => (
              <Link
                key={idx}
                to={item.href}
                className={cn(
                  location.pathname === item.href
                    ? 'bg-gray-900 text-white'
                    : 'text-gray-300 hover:bg-gray-700 hover:text-white',
                  'block rounded-md px-3 py-2 text-base font-medium'
                )}
                aria-current='page'
              >
                {item.label}
              </Link>
            ))}
          </div>
          <div className='border-t border-gray-700 pb-3 pt-4'>
            <div className='flex items-center px-5 sm:px-6'>
              <div className='flex-shrink-0'>
                <img className='h-10 w-10 rounded-full' src={reactLogo} alt='User avatar' />
              </div>
              <div className='ml-3'>
                <div className='text-base font-medium text-white'>Tom Cook</div>
                <div className='text-sm font-medium text-gray-400'>tom@example.com</div>
              </div>
              <Button
                type='button'
                className='relative ml-auto flex-shrink-0 rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800'
              >
                <span className='absolute -inset-1.5' />
                <span className='sr-only'>View notifications</span>
                <span className='i-lucide-bell-dot h-6 w-6' />
              </Button>
            </div>
            <div className='mt-3 space-y-1 px-2 sm:px-3'>
              <Link
                to='#'
                className='block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white'
              >
                Your Profile
              </Link>
              <Link
                to='#'
                className='block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white'
              >
                Settings
              </Link>
              <Link
                to='#'
                className='block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white'
              >
                Sign out
              </Link>
            </div>
          </div>
        </div>
      </nav>
      <Outlet />
    </div>
  )
}

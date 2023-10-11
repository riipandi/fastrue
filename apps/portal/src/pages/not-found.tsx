import { Link } from 'react-router-dom'

export default function NotFoundPage() {
  return (
    <div className='flex flex-1 items-center justify-center h-full min-h-screen bg-gray-100 dark:bg-gray-950'>
      <div className='mx-auto max-w-screen-2xl px-4 md:px-8'>
        <div className='flex flex-col items-center'>
          <p className='mb-6 text-sm font-semibold uppercase text-indigo-500 md:text-base'>
            Error 404
          </p>
          <h1 className='mb-6 text-center text-2xl font-bold text-gray-800 md:text-3xl'>
            Page not found
          </h1>
          <p className='mb-12 max-w-screen-md text-center text-gray-500 md:text-lg'>
            The page you&apos;re looking for doesn&apos;t exist.
          </p>
          <Link
            to='/'
            className='block rounded-lg bg-gray-800 px-8 py-3 text-center text-sm text-white outline-none ring-gray-300 transition duration-100 hover:bg-gray-700 focus-visible:ring active:bg-gray-600 md:text-base'
          >
            Back to main page
          </Link>
        </div>
      </div>
    </div>
  )
}

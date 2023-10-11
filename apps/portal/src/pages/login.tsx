import { Link } from 'react-router-dom'

export default function LoginPage() {
  return (
    <div className='mx-auto max-w-md w-full'>
      <h2 className='mb-6 text-center text-2xl font-bold text-gray-800 md:mb-8 lg:text-3xl'>
        Continue to your account
      </h2>
      <form className='mx-auto w-full rounded-lg border bg-white shadow-lg'>
        <div className='flex flex-col gap-4 p-4 md:p-8'>
          <div>
            <label htmlFor='email' className='mb-2 inline-block text-sm text-gray-800 sm:text-base'>
              Email address
            </label>
            <input
              name='email'
              className='w-full rounded border bg-gray-50 px-3 py-2 text-gray-800 outline-none ring-primary-300 transition duration-100 focus:ring'
            />
          </div>
          <div>
            <label
              htmlFor='password'
              className='mb-2 inline-block text-sm text-gray-800 sm:text-base'
            >
              Password
            </label>
            <input
              name='password'
              className='w-full rounded border bg-gray-50 px-3 py-2 text-gray-800 outline-none ring-primary-300 transition duration-100 focus:ring'
            />
          </div>
          <div className='mt-2'>
            <button className='block w-full rounded-lg bg-gray-800 px-8 py-3 text-center text-sm font-semibold text-white outline-none ring-gray-300 transition duration-100 hover:bg-gray-700 focus-visible:ring active:bg-gray-600 md:text-base'>
              Log in
            </button>
          </div>
        </div>
        <div className='flex items-center rounded-b-lg justify-center bg-gray-50 p-4'>
          <p className='text-center text-sm text-gray-500'>
            Forgot your password?{' '}
            <Link
              to='/recover'
              className='text-primary-500 transition duration-100 hover:text-primary-600 active:text-primary-700'
            >
              Reset here
            </Link>
          </p>
        </div>
      </form>
    </div>
  )
}

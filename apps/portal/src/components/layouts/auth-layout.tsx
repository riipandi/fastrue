import { Outlet } from 'react-router-dom'

export default function AuthLayout() {
  return (
    <div className='flex flex-1 items-center justify-center h-full min-h-screen bg-gray-100 dark:bg-gray-950'>
      <Outlet />
    </div>
  )
}

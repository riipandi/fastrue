import { Outlet } from 'react-router-dom'

export default function UserLayout() {
  return (
    <div className='bg-gray-100 dark:bg-gray-950 w-full min-h-screen h-full'>
      <Outlet />
    </div>
  )
}

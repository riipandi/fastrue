import { Navigate, useRoutes } from 'react-router-dom'

import { AppLayout, AuthLayout } from '@/components/Layouts'
import { withAdmin, withLoggedIn, withLoggedOut } from '@/hooks/AuthContext'
import { useAuthentication } from '@/hooks/AuthProvider'
import Error404 from '@/pages/404'
import { AdminDashboard, AdminLogin } from '@/pages/admin'
import { Login, Recovery, ResetPassword } from '@/pages/auth'
import { UserDashboard } from '@/pages/users'

export const AppRoutes = () => {
  return useRoutes([
    { path: '/ui', element: <HomePage /> },
    {
      element: <AuthLayout />,
      children: [
        { path: '/ui/login', element: withLoggedOut(Login)() },
        { path: '/ui/recovery', element: withLoggedOut(Recovery)() },
        { path: '/ui/reset-password', element: withLoggedOut(ResetPassword)() },
        { path: '/ui/admin/login', element: withLoggedOut(AdminLogin)() },
      ],
    },
    {
      element: <AppLayout />,
      children: [
        { path: '/ui/dashboard', element: withLoggedIn(UserDashboard)() },
        { path: '/ui/admin', element: withAdmin(AdminDashboard)() },
      ],
    },
    { path: '*', element: <Error404 /> },
  ])
}

export function HomePage() {
  const { loggedIn, isAdmin } = useAuthentication()

  if (!loggedIn) {
    return <Navigate replace to='/ui/login' />
  }

  if (loggedIn && isAdmin) {
    return <Navigate replace to='/ui/admin' />
  }

  return <Navigate replace to='/ui/dashboard' />
}

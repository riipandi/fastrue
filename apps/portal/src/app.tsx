import { Navigate, useRoutes } from 'react-router-dom'

// import useAuth from '@/context/hooks/useAuth'
import AdminLayout from '@/components/layouts/admin-layout'
import UserLayout from '@/components/layouts/user-layout'
import AuthLayout from '@/components/layouts/auth-layout'

import ActivityLogsPage from '@/pages/activity-logs'
import ManageUsersPage from '@/pages/manage-users'
import OverviewPage from '@/pages/overview'
import SettingsPage from '@/pages/settings'
import AccountPage from '@/pages/account'
import LoginPage from '@/pages/login'
import NotFoundPage from '@/pages/not-found'

const AppRoutes = () => {
  return useRoutes([
    { path: '/', element: <Navigate to='/login' replace /> },
    {
      element: <UserLayout />,
      children: [{ path: '/account', element: <AccountPage /> }],
    },
    {
      path: '/admin',
      element: <AdminLayout />,
      children: [
        { path: '/admin', element: <OverviewPage /> },
        { path: '/admin/manage-users', element: <ManageUsersPage /> },
        { path: '/admin/activity-logs', element: <ActivityLogsPage /> },
        { path: '/admin/settings', element: <SettingsPage /> },
      ],
    },
    {
      element: <AuthLayout />,
      children: [{ path: '/login', element: <LoginPage /> }],
    },
    { path: '*', element: <NotFoundPage /> },
  ])
}

export default function App() {
  // const { authenticated } = useAuth()

  // if (!authenticated) {
  //   return <Navigate to='/login' replace />
  // }

  return <AppRoutes />
}

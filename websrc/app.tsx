import { BrowserRouter } from 'react-router-dom'
import { clsx } from 'clsx'
import { Toaster } from 'sonner'

import { AuthProvider } from '@/hooks/AuthProvider'
import { AppRoutes } from '@/routes'

export default function App() {
  return (
    <div className={clsx(import.meta.env.DEV ? 'debug-screens' : '')}>
      <BrowserRouter basename='/_/'>
        <AuthProvider>
          <AppRoutes />
          <Toaster richColors position='bottom-right' duration={3000} />
        </AuthProvider>
      </BrowserRouter>
    </div>
  )
}

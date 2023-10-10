import { auth } from '@/utils/fastrue'
import { useEffect, useState } from 'react'

export default function useAuth() {
  const [authenticated, setAuthenticated] = useState<boolean>(false)

  useEffect(() => {
    const sessionCheck = async () => {
      // TODO replace with real session checking
      const resp = await auth.settings()
      setAuthenticated(resp.providers.email)
    }

    sessionCheck()
    return () => {}
  }, [])

  return { authenticated }
}

import Fastrue from '@fastrue/client-js'

/**
 * Instantiate the Fastrue auth client with configuration
 */
export const auth = new Fastrue({
  // The absolute path of the Fastrue endpoint
  apiUrl: 'http://localhost:8090/auth',
  audience: 'webapp',
  setCookie: false,
})

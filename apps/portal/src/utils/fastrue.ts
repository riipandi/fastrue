import Fastrue from '@fastrue/client-js'

/**
 * Instantiate the Fastrue auth client with configuration
 */
export const auth = new Fastrue({
  // The absolute path of the Fastrue endpoint
  apiUrl: '/api',
  audience: 'standalone-console',
  setCookie: false,
})

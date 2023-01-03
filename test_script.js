import { sleep } from 'k6'
import http from 'k6/http'

// export default function () {
//   http.get('http://127.0.0.1:9999/api/settings')
//   sleep(1)
// }

/**
 * Load test user authentication using k6:
 * Docs: https://k6.io/docs/get-started/running-k6
 *
 *  k6 run --vus 1000 --iterations 1000 test_script.js
 */
export default function () {
  const url = 'http://localhost:9999/api/v1/login'
  const payload = JSON.stringify({
    email: 'admin@example.com',
    password: 'passw0rd',
  })
  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  }
  http.post(url, payload, params)
  sleep(1)
}

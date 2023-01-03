// https://k6.io/docs/get-started/running-k6
import { sleep } from 'k6'
import http from 'k6/http'

export default function () {
  http.get('http://127.0.0.1:9999/api/settings')
  sleep(1)
}

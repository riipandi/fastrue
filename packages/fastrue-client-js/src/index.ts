import { $Fetch, ofetch, FetchError } from 'ofetch'

import type { SettingReponse, DummyReponse } from './types'

const HTTPRegexp = /^http:\/\//
const defaultApiURL = `/auth`

export interface FastrueOptions {
  apiUrl: string
  audience: string
  setCookie: boolean
}

interface RequestOptions extends RequestInit {
  aud?: string
}

export default class Fastrue {
  private fetcher: $Fetch
  private apiUrl: string
  private audience: string
  private setCookie: boolean

  constructor({ apiUrl = defaultApiURL, audience = '', setCookie = false }: FastrueOptions) {
    if (HTTPRegexp.test(apiUrl)) {
      console.warn(
        'Warning:\n\nDO NOT USE HTTP IN PRODUCTION FOR FASTRUE EVER!\nFastrue REQUIRES HTTPS to work securely.'
      )
    }

    this.apiUrl = apiUrl
    this.audience = audience
    this.setCookie = setCookie

    // No export is transpiled for sake of modern syntax.
    // You probably need to transpile ofetch, destr and ufo packages with babel for ES5 support.
    this.fetcher = ofetch.create({ baseURL: this.apiUrl })
  }

  private async _request<T>(path: string, options?: RequestOptions) {
    const headers: HeadersInit = new Headers(options?.headers)

    // Set default request headers
    headers.append('Accept', 'application/json')
    headers.append('Content-Type', 'application/json')

    if (options?.aud && !headers.has('X-JWT-AUD')) {
      headers.append('X-JWT-AUD', options.aud)
    } else {
      headers.append('X-JWT-AUD', this.audience)
    }

    try {
      return await this.fetcher<T>(path, { ...options, headers })
    } catch (error) {
      if (error instanceof FetchError && error.data) {
        if (error.data.message) {
          error.message = error.data.message
        }
      }
      throw error
    }
  }

  settings(): Promise<SettingReponse> {
    return this._request<SettingReponse>('/settings')
  }

  signup(params: { email: string; password?: string; data?: object }): Promise<DummyReponse> {
    return this._request<DummyReponse>('/signup', {
      method: 'POST',
      body: JSON.stringify(params),
    })
  }

  // login(params: { email: string; password: string; remember: boolean }) {
  //   const username = encodeURIComponent(params.email)
  //   const password = encodeURIComponent(params.password)
  //   return this._request('/token', {
  //     method: 'POST',
  //     headers: {
  //       'Content-Type': 'application/x-www-form-urlencoded',
  //       'X-Use-Cookie': params.remember ? '1' : 'session',
  //     },
  //     body: `grant_type=password&username=${username}&password=${password}`,
  //   }).then((response) => {
  //     // User.removeSavedSession()
  //     // return this.createUser(response, params.remember)
  //   })
  // }

  // loginExternalUrl(provider) {
  //   return `${this.api.apiURL}/authorize?provider=${provider}`
  // }

  // confirm(token, remember) {
  //   this._setRememberHeaders(remember)
  //   return this.verify('signup', token, remember)
  // }

  // requestPasswordRecovery(email) {
  //   return this._request('/recover', {
  //     method: 'POST',
  //     body: JSON.stringify({ email }),
  //   })
  // }

  // recover(token, remember) {
  //   this._setRememberHeaders(remember)
  //   return this.verify('recovery', token, remember)
  // }

  // acceptInvite(token, password, remember) {
  //   this._setRememberHeaders(remember)
  //   return this._request('/verify', {
  //     method: 'POST',
  //     body: JSON.stringify({ token, password, type: 'signup' }),
  //   }).then((response) => this.createUser(response, remember))
  // }

  // acceptInviteExternalUrl(provider, token) {
  //   return `${this.api.apiURL}/authorize?provider=${provider}&invite_token=${token}`
  // }

  // createUser(tokenResponse, remember = false) {
  //   this._setRememberHeaders(remember)
  //   const user = new User(this.api, tokenResponse, this.audience)
  //   return user.getUserData().then((userData) => {
  //     if (remember) {
  //       userData._saveSession()
  //     }
  //     return userData
  //   })
  // }

  // currentUser() {
  //   const user = User.recoverSession(this.api)
  //   user && this._setRememberHeaders(user._fromStorage)
  //   return user
  // }

  // verify(type, token, remember) {
  //   this._setRememberHeaders(remember)
  //   return this._request('/verify', {
  //     method: 'POST',
  //     body: JSON.stringify({ token, type }),
  //   }).then((response) => this.createUser(response, remember))
  // }

  // _setRememberHeaders(remember: boolean) {
  //   if (this.setCookie) {
  //     this.api.defaultHeaders = this.api.defaultHeaders || {}
  //     this.api.defaultHeaders['X-Use-Cookie'] = remember ? '1' : 'session'
  //   }
  // }
}

// if (typeof window !== 'undefined') {
//   window.Fastrue = Fastrue
// }

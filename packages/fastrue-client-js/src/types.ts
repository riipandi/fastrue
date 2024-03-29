export type SettingReponse = {
  description: string
  providers: {
    email: boolean
    google: boolean
    github: boolean
  }
  disable_signup: boolean
  autoconfirm: boolean
}

export type DummyReponse = {
  description: string
  headers: object
  query: object
  body: object
  params: object
}

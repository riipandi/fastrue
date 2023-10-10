import { TimeStamps } from './_common'

export interface Identity extends TimeStamps {
  id: string
  user_id: string // UUID
  identity_data: object
  provider: string
  email: string
  last_sign_in_at: Date | string
}

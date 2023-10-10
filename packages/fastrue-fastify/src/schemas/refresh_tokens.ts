import { TimeStamps } from './_common'

export interface RefreshToken extends TimeStamps {
  id: string // UUID
  parent?: string
  token?: string
  user_id?: string
  revoked?: boolean
  session_id?: string // UUID
}

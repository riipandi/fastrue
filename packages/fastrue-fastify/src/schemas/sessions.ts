import { TimeStamps } from './_common'

export interface Session extends TimeStamps {
  id: string // UUID
  user_id: string // UUID
  factor_id?: string // UUID
  aal?: string // type aal_level
  not_after?: Date | string
}

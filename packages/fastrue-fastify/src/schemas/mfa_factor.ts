import { TimeStamps } from './_common'

export interface MfaFactor extends TimeStamps {
  id: string // UUID
  user_id: string // UUID
  friendly_name?: string
  factor_type: string // type factor_type
  status: string // type factor_status
  secret?: string
}

export interface MfaChallenge {
  id: string // UUID
  factor_id: string // UUID
  created_at: Date | string
  verified_at?: Date | string
  ip_address: string
}

export interface MfaAmrClaim {
  id: string // UUID
  session_id: string // UUID
  created_at: Date | string
  updated_at: Date | string
  authentication_method: string
}

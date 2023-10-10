import { TimeStamps } from './_common'

export interface FlowState extends TimeStamps {
  id: string // UUID
  user_id: string // UUID
  auth_code: string
  authentication_method: string
  code_challenge_method: string
  code_challenge: string
  provider_type: string
  provider_access_token?: string
  provider_refresh_token?: string
}

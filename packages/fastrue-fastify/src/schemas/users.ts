import { TimeStamps } from './_common'

export interface User extends TimeStamps {
  id: string // UUID
  uid: string
  aud?: string
  role?: string
  email: string // unique
  email_confirmed_at?: Date | string
  email_change_token_new?: string
  email_change?: string
  email_change_sent_at?: Date | string
  email_change_token_current?: string
  email_change_confirm_status?: number
  phone?: string
  phone_confirmed_at?: Date | string
  phone_change?: string
  phone_change_token?: string
  phone_change_sent_at?: Date | string
  invited_at?: Date | string
  confirmation_token?: string
  confirmation_sent_at?: Date | string
  recovery_token?: string
  recovery_sent_at?: Date | string
  reauthentication_token?: string
  reauthentication_sent_at?: Date | string
  last_sign_in_at?: Date | string
  raw_app_meta_data?: object
  raw_user_meta_data?: object
  is_super_admin?: boolean
  is_sso_user: boolean
  confirmed_at?: Date | string
  banned_until?: Date | string
  deleted_at?: Date | string
}

export interface Password extends TimeStamps {
  id: string // UUID
  user_id: string // UUID
  encrypted_password: string
}

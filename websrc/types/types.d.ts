/* This file is generated and managed by tsync */

interface User {
  instance_id?: Uuid
  id: Uuid
  aud?: string
  role?: string
  email: string
  encrypted_password?: string
  confirmed_at?: Date
  invited_at?: Date
  confirmation_token?: string
  confirmation_sent_at?: Date
  recovery_token?: string
  recovery_sent_at?: Date
  email_change_token?: string
  email_change?: string
  email_change_sent_at?: Date
  last_sign_in_at?: Date
  raw_app_meta_data?: Json
  raw_user_meta_data?: Json
  is_super_admin: boolean
  created_at: Date
  updated_at: Date
}

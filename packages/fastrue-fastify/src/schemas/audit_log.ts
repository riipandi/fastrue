export interface AuditLog {
  id: string // UUID
  payload?: object
  ip_address: string
  created_at: Date | string
}

import { TimeStamps } from './_common'

export interface SsoDomain extends TimeStamps {
  id: string // UUID
  sso_provider_id: string // UUID
  domain: string
}

export interface SsoProvider extends TimeStamps {
  id: string // UUID
  resource_id: string
}

export interface SamlProvider extends TimeStamps {
  id: string // UUID
  sso_provider_id: string // UUID
  entity_id: string // unique
  metadata_xml: string
  metadata_url?: string
  attribute_mapping?: object
}

export interface SamlRelayState extends TimeStamps {
  id: string // UUID
  sso_provider_id: string // UUID
  flow_state_id?: string // UUID
  request_id: string
  for_email?: string
  redirect_to?: string
  from_ip_address?: string
}

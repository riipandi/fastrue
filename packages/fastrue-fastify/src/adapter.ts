import type { Sql, PostgresError, PendingQuery } from 'postgres'
import { User } from './schemas/users'

type AdapterOptions = {
  schema?: string
  tables?: {
    audit_log?: string
    flow_state?: string
    identities?: string
    mfa_amr_claims?: string
    mfa_challenges?: string
    mfa_factors?: string
    passwords?: string
    refresh_tokens?: string
    saml_providers?: string
    saml_relay_states?: string
    sessions?: string
    sso_domains?: string
    sso_providers?: string
    users?: string
  }
}

const escapeName = (val: string) => `"${val}"`

// TODO set output type
export const postgresAdapter = (sql: Sql, opts: AdapterOptions): any => {
  const AUDIT_LOG_TABLE = escapeName(opts.tables?.audit_log || 'audit_log')
  const FLOW_STATE_TABLE = escapeName(opts.tables?.flow_state || 'flow_state')
  const IDENTITIES_TABLE = escapeName(opts.tables?.identities || 'identities')
  const MFA_AMR_CLAIMS_TABLE = escapeName(opts.tables?.mfa_amr_claims || 'mfa_amr_claims')
  const MFA_CHALLENGES_TABLE = escapeName(opts.tables?.mfa_challenges || 'mfa_challenges')
  const MFA_FACTORS_TABLE = escapeName(opts.tables?.mfa_factors || 'mfa_factors')
  const PASSWORDS_TABLE = escapeName(opts.tables?.passwords || 'passwords')
  const REFRESH_TOKENS_TABLE = escapeName(opts.tables?.refresh_tokens || 'refresh_tokens')
  const SAML_PROVIDERS_TABLE = escapeName(opts.tables?.saml_providers || 'saml_providers')
  const SAML_RELAY_STATES_TABLE = escapeName(opts.tables?.saml_relay_states || 'saml_relay_states')
  const SESSIONS_TABLE = escapeName(opts.tables?.sessions || 'sessions')
  const SSO_DOMAINS_TABLE = escapeName(opts.tables?.sso_domains || 'sso_domains')
  const SSO_PROVIDERS_TABLE = escapeName(opts.tables?.sso_providers || 'sso_providers')
  const USERS_TABLE = escapeName(opts.tables?.users || 'users')

  return () => {
    return {
      getAllUsers: async () => {
        console.log('getAllUsers called')
      },
    }
  }
}

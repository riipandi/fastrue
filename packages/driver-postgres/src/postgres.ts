import type { Sql, PostgresError, PendingQuery } from 'postgres'

type AdapterOptions = {
  schema: string
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

  // return (LuciaError) => {
  return {
    getUser: async (userId: string) => {},
    setUser: async (user: string, key: string) => {},
    deleteUser: async (userId: string) => {},
    updateUser: async (userId: string, partialUser: string) => {},
    getSession: async (sessionId: string) => {},
    getSessionsByUserId: async (userId: string) => {},
    setSession: async (session: string) => {},
    deleteSession: async (sessionId: string) => {},
    deleteSessionsByUserId: async (userId: string) => {},
    updateSession: async (sessionId: string, partialSession: string) => {},
    getKey: async (keyId: string) => {},
    getKeysByUserId: async (userId: string) => {},
    setKey: async (key: string) => {},
    deleteKey: async (keyId: string) => {},
    deleteKeysByUserId: async (userId: string) => {},
    updateKey: async (keyId: string, partialKey: string) => {},
    getSessionAndUser: async (sessionId: string) => {},
  }
  // }
}

export async function get<_Schema extends {}>(queryPromise: PendingQuery<_Schema[]>) {
  const result = await queryPromise
  return result.at(0) ?? null
}

export async function getAll<_Schema extends {}>(queryPromise: PendingQuery<_Schema[]>) {
  return await queryPromise
}

function processException(e: any) {
  return e as Partial<PostgresError>
}

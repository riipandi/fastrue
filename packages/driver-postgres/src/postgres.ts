import type { Sql, PostgresError, PendingQuery } from 'postgres'

type AdapterOptions = {
  schema: string
  tables: {
    user: string
    session: string | null
    key: string
  }
}

const ESCAPE_CHAR = `"`

const escapeName = (val: string) => {
  return `${ESCAPE_CHAR}${val}${ESCAPE_CHAR}`
}

// TODO set output type
export const postgresAdapter = (sql: Sql, opts: AdapterOptions): any => {
  const ESCAPED_USER_TABLE_NAME = escapeName(opts.tables.user)
  const ESCAPED_SESSION_TABLE_NAME = opts.tables.session ? escapeName(opts.tables.session) : null
  const ESCAPED_KEY_TABLE_NAME = escapeName(opts.tables.key)

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

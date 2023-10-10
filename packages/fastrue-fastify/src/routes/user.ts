import { FastifyPluginAsync } from 'fastify'
import postgres from 'postgres'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  const sql = postgres('postgres://postgres:postgres@127.0.0.1:5432/fastrue?sslmode=disable')

  fastify.get(
    '/user',
    {
      schema: {
        summary: 'Fetch the latest user account information',
        description: 'Put long description here...',
        tags: ['account'],
      },
    },
    async (request, _reply) => {
      const data = await sql`select * from users`
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
        data,
      }
    }
  )

  fastify.put(
    '/user',
    {
      schema: {
        summary: 'Update certain properties of the current user account',
        description: 'Put long description here...',
        tags: ['account'],
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )
}

export default routes

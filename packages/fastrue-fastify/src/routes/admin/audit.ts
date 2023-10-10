import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/audit',
    {
      schema: {
        summary: 'Fetch audit log events',
        description: 'Put long description here...',
        tags: ['admin'],
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

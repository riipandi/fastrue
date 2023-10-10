import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/signup',
    {
      schema: {
        summary: 'Signs a user up, create a new user',
        description: 'Put long description here...',
        tags: ['Authentication'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
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

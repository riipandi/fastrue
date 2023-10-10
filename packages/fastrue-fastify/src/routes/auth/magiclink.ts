import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/magiclink',
    {
      schema: {
        summary: 'Authenticate a user by sending them a magic link',
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

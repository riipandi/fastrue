import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/reauthenticate',
    {
      schema: {
        summary: 'Reauthenticates for the purpose of password change',
        description:
          'Reauthenticates the possession of an email or phone number for the purpose of password change',
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

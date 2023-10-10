import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/token',
    {
      schema: {
        summary: 'Issues access and refresh tokens based on grant type',
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
      const result = {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
      reply.code(200).header('Content-Type', 'application/json; charset=utf-8').send(result)
    }
  )
}

export default routes

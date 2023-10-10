import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/otp',
    {
      schema: {
        summary: 'Authenticate a user by sending them a One-Time Password over email or SMS',
        description: 'Put long description here...',
        tags: ['auth'],
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

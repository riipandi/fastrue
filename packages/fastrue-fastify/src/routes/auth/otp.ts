import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post('/otp', opts, async (request, reply) => {
    return {
      description: 'Authenticate a user by sending them a One-Time Password over email or SMS',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

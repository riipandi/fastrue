import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post('/token', opts, async (request, reply) => {
    return {
      description: 'Issues access and refresh tokens based on grant type',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

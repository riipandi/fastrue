import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post('/sso', opts, async (request, reply) => {
    return {
      description: 'Initiate a Single-Sign On flow',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

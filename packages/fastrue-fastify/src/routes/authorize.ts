import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/authorize', opts, async (request, reply) => {
    return {
      description: 'Redirects to an external OAuth provider. Usually for use as clickable links',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

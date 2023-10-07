import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/verify', opts, async (request, reply) => {
    return {
      description:
        'Authenticate by verifying the posession of a one-time token. Usually for use as clickable links',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
  fastify.post('/verify', opts, async (request, reply) => {
    return {
      description: 'Authenticate by verifying the posession of a one-time token',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

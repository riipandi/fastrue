import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/callback', opts, async (request, reply) => {
    return {
      description: 'Redirects OAuth flow errors to the frontend app',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
  fastify.post('/callback', opts, async (request, reply) => {
    return {
      description: 'Redirects OAuth flow errors to the frontend app',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

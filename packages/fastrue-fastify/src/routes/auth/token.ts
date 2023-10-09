import { FastifyPluginAsync } from 'fastify'

const description = 'Issues access and refresh tokens based on grant type'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post('/token', opts, async (request, reply) => {
    const result = {
      description,
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
    reply.code(200).header('Content-Type', 'application/json; charset=utf-8').send(result)
  })
}

export default routes

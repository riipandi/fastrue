import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/token',
    {
      schema: {
        summary: 'Issues access and refresh tokens based on grant type.',
        description: 'Put long description here...',
        tags: ['auth', 'oauth'],
        required: ['grant_type'],
      },
    },
    async (request, reply) => {
      // const select = fastify.db
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

import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/user', opts, async (request, _reply) => {
    return {
      description: 'Fetch the latest user account information',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.put('/user', opts, async (request, reply) => {
    return {
      description: 'Update certain properties of the current user account',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes
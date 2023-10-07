import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/users', opts, async (request, reply) => {
    return {
      description: 'Fetch a listing of users',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.get('/users/:userId', opts, async (request, reply) => {
    return {
      description: 'Fetch user account data for a user',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.put('/users/:userId', opts, async (request, reply) => {
    return {
      description: 'Update user account data',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.delete('/users/:userId', opts, async (request, reply) => {
    return {
      description: 'Delete a user',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/users/:userId/factors', opts, async (request, reply) => {
    return {
      description: 'List all of the MFA factors for a user',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.put('/users/:userId/factors/:factorId', opts, async (request, reply) => {
    return {
      description: 'Update a user MFA factor',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.delete('/users/:userId/factors/:factorId', opts, async (request, reply) => {
    return {
      description: 'Remove a user MFA factor',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

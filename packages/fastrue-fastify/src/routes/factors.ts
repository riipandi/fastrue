import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post('/factors', opts, async (request, reply) => {
    return {
      description: 'Begin enrolling a new factor for MFA',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.post('/factors/:factorId/challenge', opts, async (request, reply) => {
    return {
      description: 'Create a new challenge for a MFA factor',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.post('/factors/:factorId/verify', opts, async (request, reply) => {
    return {
      description: 'Verify a challenge on a factor',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.delete('/factors/:factorId', opts, async (request, reply) => {
    return {
      description: 'Remove a MFA factor from a user',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

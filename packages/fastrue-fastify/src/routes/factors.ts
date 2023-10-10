import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/factors',
    {
      schema: {
        summary: 'Begin enrolling a new factor for MFA',
        description: 'Put long description here...',
        tags: ['account'],
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )

  fastify.post(
    '/factors/:factorId/challenge',
    {
      schema: {
        summary: 'Create a new challenge for a MFA factor',
        description: 'Put long description here...',
        tags: ['account'],
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )

  fastify.post(
    '/factors/:factorId/verify',
    {
      schema: {
        summary: 'Verify a challenge on a factor',
        description: 'Put long description here...',
        tags: ['account'],
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )

  fastify.delete(
    '/factors/:factorId',
    {
      schema: {
        summary: 'Remove a MFA factor from a user',
        description: 'Put long description here...',
        tags: ['account'],
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )
}

export default routes

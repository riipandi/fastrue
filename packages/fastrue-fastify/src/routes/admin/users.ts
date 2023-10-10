import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/users',
    {
      schema: {
        summary: 'Fetch a listing of users',
        description: 'Put long description here...',
        tags: ['Administration'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
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

  fastify.get(
    '/users/:userId',
    {
      schema: {
        summary: 'Fetch user account data for a user',
        description: 'Put long description here...',
        tags: ['Administration'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
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

  fastify.put(
    '/users/:userId',
    {
      schema: {
        summary: 'Update user account data',
        description: 'Put long description here...',
        tags: ['Administration'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
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
    '/users/:userId',
    {
      schema: {
        summary: 'Delete a user',
        description: 'Put long description here...',
        tags: ['Administration'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
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

import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/users/:userId/factors',
    {
      schema: {
        summary: 'List all of the MFA factors for a user',
        description: 'Put long description here...',
        tags: ['Administration'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {},
        //   },
        // },
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
    '/users/:userId/factors/:factorId',
    {
      schema: {
        summary: 'Fetch audit log events',
        description: 'Put long description here...',
        tags: ['Administration'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {},
        //   },
        // },
      },
    },
    async (request, reply) => {
      return {
        description: 'Update a user MFA factor',
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )

  fastify.delete(
    '/users/:userId/factors/:factorId',
    {
      schema: {
        summary: 'Removea user MFA factor',
        description: 'Put long description here...',
        tags: ['Administration'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {},
        //   },
        // },
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

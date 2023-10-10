import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/user',
    {
      schema: {
        summary: 'Fetch the latest user account information',
        description: 'Put long description here...',
        tags: ['Account'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {},
        //   },
        // },
      },
    },
    async (request, _reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )

  fastify.put(
    '/user',
    {
      schema: {
        summary: 'Update certain properties of the current user account',
        description: 'Put long description here...',
        tags: ['Account'],
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

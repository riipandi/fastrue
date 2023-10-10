import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/verify',
    {
      schema: {
        summary:
          'Authenticate by verifying the posession of a one-time token. Usually for use as clickable links',
        description: 'Put long description here...',
        tags: ['Authentication'],
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
  fastify.post(
    '/verify',
    {
      schema: {
        summary: 'Authenticate by verifying the posession of a one-time token',
        description: 'Put long description here...',
        tags: ['Authentication'],
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

import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/callback',
    {
      schema: {
        summary: 'Redirect OAuth flow errors to the frontend app',
        description: 'Put long description here...',
        tags: ['Authentication'],
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
  fastify.post(
    '/callback',
    {
      schema: {
        summary: 'Redirect OAuth flow errors to the frontend app',
        description: 'Put long description here...',
        tags: ['Authentication'],
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

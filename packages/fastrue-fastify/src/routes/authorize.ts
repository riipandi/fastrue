import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/authorize',
    {
      schema: {
        summary: 'Redirects to an external OAuth provider',
        description: 'Usually for use as clickable links.',
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

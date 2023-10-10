import { FastifyPluginAsync, type RouteShorthandOptions } from 'fastify'

const routeOptions: RouteShorthandOptions = {}

const routes: FastifyPluginAsync = async (fastify, _opts): Promise<void> => {
  fastify.get(
    '/settings',
    {
      schema: {
        summary: 'Retrieve some of the public settings of the server',
        description:
          'Use this endpoint to configure parts of any authentication UIs depending on the configured settings.',
        tags: ['general'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {
        //       name: { type: 'string' },
        //       version: { type: 'string' },
        //       description: { type: 'string' },
        //     },
        //   },
        // },
      },
    },
    async (request, reply) => {
      const result = {
        providers: {
          email: true,
          google: false,
          github: false,
        },
        disable_signup: false,
        autoconfirm: false,
      }
      reply.code(200).header('Content-Type', 'application/json; charset=utf-8').send(result)
    }
  )
}

export default routes

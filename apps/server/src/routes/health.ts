import { FastifyPluginAsync, RouteShorthandOptions } from 'fastify'

const opts: RouteShorthandOptions = {
  schema: {
    response: {
      200: {
        type: 'object',
        properties: {
          name: { type: 'string' },
          version: { type: 'string' },
          description: { type: 'string' },
        },
      },
    },
  },
}

const route: FastifyPluginAsync = async (fastify, _opts): Promise<void> => {
  fastify.get('/health', opts, async (_request, reply) => {
    reply.code(200).header('Content-Type', 'application/json; charset=utf-8').send({
      version: 'v0.0.0',
      name: 'Fastrue',
      description: 'Fastrue is a fast and robust authentication library',
    })
  })
}

export default route

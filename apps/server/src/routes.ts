import { FastifyPluginAsync, RouteShorthandOptions } from 'fastify'

const defaultApiRouteOpts: RouteShorthandOptions = {
  schema: {
    response: {
      200: {
        type: 'object',
        properties: {
          message: { type: 'string' },
        },
      },
    },
  },
}

export const defaultApiRoute: FastifyPluginAsync = async (fastify, _opts): Promise<void> => {
  fastify.get('/', defaultApiRouteOpts, async (_request, reply) => {
    reply.code(200).send({ message: 'All is well' })
  })
}

const healthCheckOpts: RouteShorthandOptions = {
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

export const healthCheckRoute: FastifyPluginAsync = async (fastify, _opts): Promise<void> => {
  fastify.get('/health', healthCheckOpts, async (_request, reply) => {
    reply.code(200).send({
      version: 'v0.0.0',
      name: 'Fastrue',
      description: 'Fastrue is a fast and robust authentication library',
    })
  })
}

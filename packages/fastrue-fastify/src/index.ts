import type { FastifyPluginAsync, FastifyRequest } from 'fastify'
import fastifyPlugin from 'fastify-plugin'

type FastrueOptions = {
  driver?: 'pg' | 'postgres'
  dbSchema?: string
  routePrefix?: string
}

const plugin: FastifyPluginAsync<FastrueOptions> = async (fastify, options = {}): Promise<void> => {
  const { driver = 'pg', dbSchema = 'public', routePrefix = '' } = options

  fastify.addHook('onRequest', async (_request) => {})

  fastify.addHook('preHandler', async (_request) => {})

  fastify.addHook('onResponse', async (_request, _reply) => {})

  // Register authentication routes
  fastify.register(import('./routes/settings'), { prefix: routePrefix })
  fastify.register(import('./routes/login'), { prefix: routePrefix })
  fastify.register(import('./routes/user'), { prefix: routePrefix })
}

export default fastifyPlugin(plugin, {
  fastify: '4.x',
  name: 'fastrue',
})

export type { FastrueOptions }

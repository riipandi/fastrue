import type { FastifyPluginAsync, FastifyRequest } from 'fastify'
import fastifyPlugin from 'fastify-plugin'
import loginRoute from './routes/login'
import settingRoutes from './routes/settings'

export type FastrueOptions = {
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
  fastify.register(settingRoutes, { prefix: routePrefix })
  fastify.register(loginRoute, { prefix: routePrefix })
}

export default fastifyPlugin(plugin, {
  fastify: '4.x',
  name: 'fastrue',
})

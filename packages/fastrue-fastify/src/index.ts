import type { FastifyPluginAsync, FastifyRequest } from 'fastify'
import fastifyPlugin from 'fastify-plugin'

type FastrueOptions = {
  driver?: 'pg' | 'postgres'
  dbSchema?: string
  routePrefix?: string
}

const plugin: FastifyPluginAsync<FastrueOptions> = async (fastify, options = {}): Promise<void> => {
  const { driver = 'pg', dbSchema = 'public', routePrefix = '' } = options
  const adminRoutePrefix = `${routePrefix}/admin`

  fastify.addHook('onRequest', async (_request) => {})

  fastify.addHook('preHandler', async (_request) => {})

  fastify.addHook('onResponse', async (request, reply) => {})

  // Register authentication routes
  fastify.register(import('./routes/auth/magiclink'), { prefix: routePrefix })
  fastify.register(import('./routes/auth/otp'), { prefix: routePrefix })
  fastify.register(import('./routes/auth/recover'), { prefix: routePrefix })
  fastify.register(import('./routes/auth/resend'), { prefix: routePrefix })
  fastify.register(import('./routes/auth/signup'), { prefix: routePrefix })
  fastify.register(import('./routes/auth/token'), { prefix: routePrefix })
  fastify.register(import('./routes/auth/verify'), { prefix: routePrefix })
  fastify.register(import('./routes/authorize'), { prefix: routePrefix })
  fastify.register(import('./routes/callback'), { prefix: routePrefix })
  fastify.register(import('./routes/factors'), { prefix: routePrefix })
  fastify.register(import('./routes/generate_link'), { prefix: routePrefix })
  fastify.register(import('./routes/invite'), { prefix: routePrefix })
  fastify.register(import('./routes/logout'), { prefix: routePrefix })
  fastify.register(import('./routes/reauthenticate'), { prefix: routePrefix })
  fastify.register(import('./routes/saml'), { prefix: routePrefix })
  fastify.register(import('./routes/settings'), { prefix: routePrefix })
  fastify.register(import('./routes/sso'), { prefix: routePrefix })
  fastify.register(import('./routes/user'), { prefix: routePrefix })

  // Register administration routes
  fastify.register(import('./routes/admin/audit'), { prefix: adminRoutePrefix })
  fastify.register(import('./routes/admin/factors'), { prefix: adminRoutePrefix })
  fastify.register(import('./routes/admin/sso'), { prefix: adminRoutePrefix })
  fastify.register(import('./routes/admin/users'), { prefix: adminRoutePrefix })
}

export default fastifyPlugin(plugin, {
  fastify: '4.x',
  name: 'fastrue',
})

export type { FastrueOptions }

import { join } from 'node:path'

import Fastify, { FastifyInstance, FastifyPluginAsync } from 'fastify'
import AutoLoad, { AutoloadPluginOptions } from '@fastify/autoload'
import fastifyRequestLogger from '@mgcrea/fastify-request-logger'
import fastifyAuthPlugin from '@fastrue/fastify'

export type AppOptions = {
  // Place your custom options for app below here.
} & Partial<AutoloadPluginOptions>

// Pass --options via CLI arguments in command to enable these options.
const options: AppOptions = {}

const server: FastifyInstance = Fastify({
  logger: {
    level: 'debug',
    transport: {
      target: '@mgcrea/pino-pretty-compact',
      options: {
        translateTime: 'HH:MM:ss Z',
        ignore: 'pid,hostname',
      },
    },
  },
})

// Register fastify plugins
server.register(fastifyRequestLogger)

// Register Fastrue plugin
server.register(fastifyAuthPlugin, {
  driver: 'postgres',
  routePrefix: '/auth',
  dbSchema: 'public',
})

server.register(AutoLoad, {
  dir: join(__dirname, 'plugins'),
})

server.register(AutoLoad, {
  dir: join(__dirname, 'routes'),
  dirNameRoutePrefix: true, // lack of prefix will mean no prefix, instead of directory name
})

const start = async () => {
  try {
    const port = Number(process.env.PORT) || 8090
    await server.listen({ port })
  } catch (err) {
    server.log.error(err)
    process.exit(1)
  }
}

start()

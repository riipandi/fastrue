import Fastify, { FastifyInstance, FastifyPluginAsync } from 'fastify'
import AutoLoad, { AutoloadPluginOptions } from '@fastify/autoload'
import fastifyRequestLogger from '@mgcrea/fastify-request-logger'
import { join } from 'path'

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

server.register(AutoLoad, {
  dir: join(__dirname, 'plugins'),
})

server.register(AutoLoad, {
  dir: join(__dirname, 'routes'),
  dirNameRoutePrefix: true, // lack of prefix will mean no prefix, instead of directory name
})

const start = async () => {
  try {
    await server.listen({ port: 3000 })
    const address = server.server.address()
    const port = typeof address === 'string' ? address : address?.port
  } catch (err) {
    server.log.error(err)
    process.exit(1)
  }
}

start()

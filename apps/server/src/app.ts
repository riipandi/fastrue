import path, { join } from 'node:path'

import Fastify, { FastifyInstance } from 'fastify'
import AutoLoad, { AutoloadPluginOptions } from '@fastify/autoload'
import fastifyRequestLogger from '@mgcrea/fastify-request-logger'
import fastifyStaticPlugin from '@fastify/static'
import fastifyAuthPlugin, { FastrueOptions } from '@fastrue/fastify'

import postgres from 'postgres'
import { defaultApiRoute, healthCheckRoute } from './routes'

export type AppOptions = {
  fastrue: FastrueOptions
} & Partial<AutoloadPluginOptions>

// Postgres database client
// TODO replace with the value from envars
const sql = postgres('postgres://postgres:postgres@127.0.0.1:5432/fastrue?sslmode=disable')

// Pass --options via CLI arguments in command to enable these options.
const options: AppOptions = {
  fastrue: {
    driver: 'postgres',
    dbClient: sql,
    dbSchema: 'public',
    routePrefix: '/api',
  },
}

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

// Register route for root endpoint
server.register(defaultApiRoute, {
  prefix: options.fastrue.routePrefix || '/api',
})

// Register route for service health check
server.register(healthCheckRoute, {
  prefix: options.fastrue.routePrefix || '/api',
})

server.register(AutoLoad, {
  dir: join(__dirname, 'plugins'),
})

// Register Fastrue plugin
server.register(fastifyAuthPlugin, options.fastrue)

server.register(fastifyStaticPlugin, {
  root: path.join(__dirname, '../public'),
  index: false,
  list: false,
  wildcard: true,
  allowedPath(pathName: string, _root, _request) {
    return !pathName.startsWith(options.fastrue.routePrefix || '/api')
  },
})

// Not found handler
server.setNotFoundHandler((req, res) => {
  if (
    req.raw.url &&
    !req.raw.url.startsWith(options.fastrue.routePrefix || '/api') &&
    !req.raw.url.startsWith(options.fastrue.routePrefix || '/swagger')
  ) {
    return res.status(200).sendFile('index.html')
  }
  res.status(404).send({
    statusCode: 404,
    error: 'Not Found',
    message: `Route ${req.method}:${req.raw.url} not found`,
  })
})

const start = async () => {
  const port = Number(process.env.PORT) || 8090
  server.listen({ port, host: '0.0.0.0' }, (err, _address) => {
    if (err) {
      server.log.error(err)
      process.exit(1)
    }
  })
}

start()

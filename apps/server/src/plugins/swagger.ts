import fp from 'fastify-plugin'
import FastifySwaggerUi, { FastifySwaggerUiOptions } from '@fastify/swagger-ui'
import fastifySwagger, { FastifySwaggerOptions } from '@fastify/swagger'

export default fp<FastifySwaggerUiOptions | FastifySwaggerOptions>(async (fastify) => {
  await fastify.register(fastifySwagger, {
    openapi: {
      info: {
        title: 'Fastrue API documentation',
        description:
          'A fast and robust authentication library powered by Fastify, inspired from by Supabase GoTrue (originally from Netlify GoTrue).',
        version: '0.1.0',
      },
      externalDocs: {
        url: 'https://fastrue.ripandis.com/docs',
        description: 'Find more info here',
      },
      servers: [
        {
          url: 'http://localhost:8090/api',
          variables: '',
        },
      ],
      components: {
        securitySchemes: {
          apiKey: {
            type: 'apiKey',
            name: 'apiKey',
            in: 'header',
          },
        },
      },
      security: [[]],
      tags: [
        { name: 'Authentication', description: 'Authentication related end-points' },
        { name: 'Account', description: 'Account related end-points' },
        { name: 'General', description: 'General related end-points' },
        { name: 'Recovery', description: 'Recovery related end-points' },
        { name: 'SAML', description: 'SAML related end-points' },
        { name: 'MFA', description: 'MFA related end-points' },
        { name: 'Administration', description: 'Administration related end-points' },
      ],
    },
    hideUntagged: true,
    exposeRoute: true,
  })

  fastify.register(FastifySwaggerUi, {
    routePrefix: '/swagger',
    uiConfig: {
      layout: 'BaseLayout',
      docExpansion: 'none',
      deepLinking: false,
    },
    uiHooks: {
      onRequest: function (request, reply, next) {
        next()
      },
      preHandler: function (request, reply, next) {
        next()
      },
    },
    staticCSP: true,
    transformStaticCSP: (header) => header,
    transformSpecification: (swaggerObject, request, reply) => {
      return swaggerObject
    },
    transformSpecificationClone: true,
  })
})

import fp from 'fastify-plugin'
import FastifySwaggerUi, { FastifySwaggerUiOptions } from '@fastify/swagger-ui'
import fastifySwagger, { FastifySwaggerOptions } from '@fastify/swagger'

const apiDescription =
  'A fast and robust authentication library powered by Fastify, inspired from by Supabase GoTrue (originally from Netlify GoTrue).'

export default fp<FastifySwaggerUiOptions | FastifySwaggerOptions>(async (fastify) => {
  await fastify.register(fastifySwagger, {
    openapi: {
      info: {
        title: 'Fastrue API documentation',
        description: apiDescription,
        version: '0.1.0',
      },
      externalDocs: {
        url: 'https://fastrue.ripandis.com/docs',
        description: 'Find more info here',
      },
      servers: [
        {
          url: 'http://localhost:8090/api',
          variables: [],
        },
        {
          url: 'https://fastrue-demo.fly.dev/api',
          variables: [],
        },
      ],
      components: {
        // securitySchemes: {
        //   apiKey: {
        //     name: 'apiKey',
        //     type: 'apiKey',
        //     in: 'header',
        //   },
        // },
      },
      security: [[]],
      tags: [
        { name: 'auth', description: 'APIs for authentication and authorization.' },
        { name: 'account', description: 'APIs used by a user to manage their account.' },
        { name: 'recovery', description: 'APIs for user password recovery.' },
        { name: 'oauth', description: 'APIs fordealing with OAuth and OIDC flows.' },
        { name: 'saml', description: 'APIs for authentication using SSO providers.' },
        { name: 'admin', description: 'Administartion APIs requiring elevated access.' },
        { name: 'general', description: 'General APIs endpoint.' },
      ],
    },
    hideUntagged: true,
    exposeRoute: true,
  })

  fastify.register(FastifySwaggerUi, {
    routePrefix: '/swagger',
    uiConfig: {
      layout: 'BaseLayout',
      docExpansion: 'list',
      deepLinking: false,
    },
    uiHooks: {
      onRequest: (request, reply, next) => next(),
      preHandler: (request, reply, next) => next(),
    },
    staticCSP: true,
    transformStaticCSP: (header) => header,
    transformSpecification: (swaggerObject, request, reply) => {
      return swaggerObject
    },
    transformSpecificationClone: true,
  })
})

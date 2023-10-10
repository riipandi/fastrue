import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/saml/metadata',
    {
      schema: {
        summary: 'Returns the SAML 2.0 Metadata XML',
        description: 'Put long description here...',
        tags: ['SAML'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )
  fastify.post(
    '/saml/acs',
    {
      schema: {
        summary: 'SAML 2.0 Assertion Consumer Service (ACS) endpoint',
        description: 'Put long description here...',
        tags: ['SAML'],
        response: {
          default: {
            description: 'Default response',
            type: 'object',
            properties: {},
          },
        },
      },
    },
    async (request, reply) => {
      return {
        headers: request.headers,
        query: request.query,
        body: request.body,
        params: request.params,
      }
    }
  )
}

export default routes

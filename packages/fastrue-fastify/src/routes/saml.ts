import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/saml/metadata',
    {
      schema: {
        summary: 'Returns the SAML 2.0 Metadata XML',
        description:
          'The metadata XML can be downloaded or used for the SAML 2.0 Metadata URL discovery mechanism. This URL is the SAML 2.0 EntityID of the Service Provider implemented by this server.',
        tags: ['saml'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {},
        //   },
        // },
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
        description:
          'Implements the SAML 2.0 Assertion Consumer Service (ACS) endpoint supporting the POST and Artifact bindings.',
        tags: ['saml'],
        // response: {
        //   default: {
        //     description: 'Default response',
        //     type: 'object',
        //     properties: {},
        //   },
        // },
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

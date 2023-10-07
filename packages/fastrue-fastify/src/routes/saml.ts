import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/saml/metadata', opts, async (request, reply) => {
    return {
      description: 'Returns the SAML 2.0 Metadata XML',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
  fastify.post('/saml/acs', opts, async (request, reply) => {
    return {
      description: 'SAML 2.0 Assertion Consumer Service (ACS) endpoint',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

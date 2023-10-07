import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/sso/providers', opts, async (request, reply) => {
    return {
      description: 'Fetch a list of all registered SSO providers',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.post('/sso/providers', opts, async (request, reply) => {
    return {
      description: 'Register a new SSO provider',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.get('/sso/providers/:ssoProviderId', opts, async (request, reply) => {
    return {
      description: 'Fetch SSO provider details',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.put('/sso/providers/:ssoProviderId', opts, async (request, reply) => {
    return {
      description: 'Update details about a SSO provider',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })

  fastify.delete('/sso/providers/:ssoProviderId', opts, async (request, reply) => {
    return {
      description: 'Remove an SSO provider',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

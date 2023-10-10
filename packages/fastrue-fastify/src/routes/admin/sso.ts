import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get(
    '/sso/providers',
    {
      schema: {
        summary: 'Fetch a list of all registered SSO providers',
        description: 'Put long description here...',
        tags: ['Administration'],
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
    '/sso/providers',
    {
      schema: {
        summary: 'Register a new SSO provider',
        description: 'Put long description here...',
        tags: ['Administration'],
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

  fastify.get(
    '/sso/providers/:ssoProviderId',
    {
      schema: {
        summary: 'Fetch SSO provider details',
        description: 'Put long description here...',
        tags: ['Administration'],
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

  fastify.put(
    '/sso/providers/:ssoProviderId',
    {
      schema: {
        summary: 'Update details about a SSO provider',
        description: 'Put long description here...',
        tags: ['Administration'],
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

  fastify.delete(
    '/sso/providers/:ssoProviderId',
    {
      schema: {
        summary: 'Remove an SSO provider',
        description: 'Put long description here...',
        tags: ['Administration'],
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

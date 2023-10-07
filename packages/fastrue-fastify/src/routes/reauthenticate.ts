import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post('/reauthenticate', opts, async (request, reply) => {
    return {
      description:
        'Reauthenticates the possession of an email or phone number for the purpose of password change',
      headers: request.headers,
      query: request.query,
      body: request.body,
      params: request.params,
    }
  })
}

export default routes

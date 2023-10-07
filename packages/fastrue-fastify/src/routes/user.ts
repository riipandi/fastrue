import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/user', opts, async (request, _reply) => {
    return { message: 'nobody', headers: request.headers }
  })
}

export default routes

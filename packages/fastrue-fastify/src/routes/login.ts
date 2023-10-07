import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/login', opts, async (_request, _reply) => {
    return { message: 'auth login' }
  })
}

export default routes

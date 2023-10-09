import path from 'node:path'
import { FastifyPluginAsync } from 'fastify'

const route: FastifyPluginAsync = async (fastify, _opts): Promise<void> => {
  fastify.get('/admin', async (_request, reply) => {
    reply.status(200).sendFile('index.html', path.join(__dirname, 'admin'), {
      cacheControl: true,
    })
  })
}

export default route

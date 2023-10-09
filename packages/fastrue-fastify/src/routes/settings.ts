import { FastifyPluginAsync } from 'fastify'

const description = 'Retrieve some of the public settings of the server'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/settings', opts, async (request, reply) => {
    const result = {
      description,
      providers: {
        email: true,
        google: false,
        github: false,
      },
      disable_signup: false,
      autoconfirm: false,
    }
    reply.code(200).header('Content-Type', 'application/json; charset=utf-8').send(result)
  })
}

export default routes

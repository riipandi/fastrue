import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/settings', opts, async (request, reply) => {
    return {
      description: 'Retrieve some of the public settings of the server',
      providers: {
        email_otp: true,
        google: false,
        github: false,
      },
      disable_signup: false,
      autoconfirm: false,
    }
  })
}

export default routes

import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/settings', opts, async (_request, _reply) => {
    return {
      providers: {
        email_otp: true,
        github: false,
        google: false,
      },
      disable_signup: false,
      autoconfirm: false,
    }
  })
}

export default routes

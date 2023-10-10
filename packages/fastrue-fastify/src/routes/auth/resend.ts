import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/resend',
    {
      schema: {
        summary: 'Resend a one-time password (OTP) throught email or SMS',
        description: 'Put long description here...',
        tags: ['auth'],
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

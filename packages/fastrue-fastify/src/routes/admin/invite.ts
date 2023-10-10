import { FastifyPluginAsync } from 'fastify'

const routes: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.post(
    '/invite',
    {
      schema: {
        summary: 'Invite a user by email',
        description:
          'Sends an invitation email which contains a link that allows the user to sign-in.',
        tags: ['admin'],
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

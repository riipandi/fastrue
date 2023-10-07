import { createEnv } from '@t3-oss/env-nextjs'
import { z } from 'zod'

export const env = createEnv({
  server: {
    NODE_ENV: z.enum(['development', 'test', 'production']).default('development'),
  },
  client: {
    NEXT_PUBLIC_SITE_URL: z.string().default('http://localhost:3000'),
  },
  experimental__runtimeEnv: {
    NEXT_PUBLIC_SITE_URL: process.env.NEXT_PUBLIC_SITE_URL,
  },
})

export const siteMeta = {
  title: 'Fastrue',
  description: 'A fast authentication library, inspired from Netlify GoTrue',
  baseUrl: env.NEXT_PUBLIC_SITE_URL,
}

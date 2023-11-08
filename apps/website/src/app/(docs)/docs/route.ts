import { type NextRequest, NextResponse } from 'next/server'
import { ServerRuntime } from 'next/types'

export const runtime: ServerRuntime = 'nodejs'

export async function GET(req: NextRequest) {
  const target = new URL('/docs/introduction', req.nextUrl)
  return NextResponse.redirect(target, 302)
}

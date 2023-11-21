import { redirect } from '@sveltejs/kit'
import type { ApiError } from 'backend'
import type { Writable } from 'svelte/store'

/**
!! IMPORTANT: always await this one
*/
export async function requireAuth<T>(parent: () => Promise<{ user: T | null }>) {
  const { user } = await parent()
  if (user === null) {
    throw redirect(302, '/login')
  }
}

type Error = { title: string, message: string }
export function pushError(context: Writable<Error[]>, error: Error) {
  context.update(ers => ers.concat([error]))
}
export function pushApiError(context: Writable<Error[]>, error: ApiError) {
  const err = {
    title: error.report,
    message: `Span: ${error.span_id}, Trace: ${error.trace_id}`
  }
  context.update(ers => ers.concat([err]))
}

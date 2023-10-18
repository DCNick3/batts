import { redirect } from "@sveltejs/kit"

/**
!! IMPORTANT: always await this one
*/
export async function requireAuth<T>(parent: () => Promise<{ user: T | null }>) {
  const { user } = await parent()
  if (user === null) {
    throw redirect(302, '/login')
  }
}

import { Api } from 'backend'
import type { LayoutLoad } from './$types'
import type { UserView } from 'backend'

export const load: LayoutLoad<{ user: UserView | null }> = async ({ fetch }) => {
  const api = new Api(fetch)
  try {
    const result = await api.getMe()

    if (result.status === 'Success') {
      return { user: result.payload }
    } else {
      // TODO: error handling
      return { user: null }
    }
  } catch (error) {
    // TODO: error handling
    return { user: null }
  }
}

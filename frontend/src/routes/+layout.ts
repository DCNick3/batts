import { Api } from 'backend'
import type { LayoutLoad } from './$types'
import type { UserView, GroupView } from 'backend'

export const load: LayoutLoad<{ user: UserView | null }> = async ({ fetch }) => {
  const api = new Api(fetch)

  let user: null | UserView = null
  let userGroups: GroupView[] = []

  try {
    const result = await api.getMe()

    if (result.status === 'Success') {
      user = result.payload

      const response = await api.getUserGroups(user.id)
      if (response.status === 'Success') {
        userGroups = response.payload
      } else {
        // TODO: error handling
        console.error(response.payload)
      }

    } else {
      // TODO: error handling
      console.error(result.payload)
    }
  } catch (error) {
    // TODO: error handling
    console.error(error)
  }

  return { user, userGroups }
}

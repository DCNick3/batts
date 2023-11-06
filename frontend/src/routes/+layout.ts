import { Api } from 'backend'
import type { LayoutLoad } from './$types'
import type { UserView, GroupView } from 'backend'
import posthog from 'posthog-js'
import { browser } from '$app/environment';

export const load: LayoutLoad<{ user: UserView | null }> = async ({ fetch }) => {
  const api = new Api(fetch)

  if (browser) {
    posthog.init(
        'phc_vANccchbK5FWoseTMHFPIHIr1h3EAKHOFOXa4dN3hBn',
        { api_host: 'https://ph.batts.tatar' }
    )
  }

  let user: null | UserView = null
  let userGroups: GroupView[] = []

  try {
    const result = await api.getMe()

    if (result.status === 'Success') {
      user = result.payload

      const response = await api.getUserGroups(user.id)
      if (response.status === 'Success') {
        userGroups = response.payload.payload
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

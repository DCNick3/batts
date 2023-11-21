import { Api } from 'backend'
import type { LayoutLoad } from './$types'
import type { UserView, GroupView, ApiError } from 'backend'
import posthog from 'posthog-js'
import { browser } from '$app/environment';

type Error
  = { type: 'Api', error: ApiError }
  | { type: 'Other', error: { title: string, message: string }}
  | null

export const load: LayoutLoad<{ user: UserView | null }> = async ({ fetch }) => {
  const api = new Api(fetch)

  let error: Error = null

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
        console.error(response.payload)
        error = { type: 'Api', error: response.payload }
      }

    } else {
      console.error(result.payload)
      error = { type: 'Api', error: result.payload }
    }
  } catch (e) {
    console.error(error)
    error = { type: 'Other', error: { title: 'Unexpected error', message: (e as any)?.message || ''}}
  }

  return { user, userGroups, error }
}

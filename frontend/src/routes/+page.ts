import { getReceivers } from '$lib/mocks/database'
import { requireAuth } from '$lib/utils'
import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { ApiError, TicketListingViewExpandedItem, GroupProfileView, UserProfileView } from 'backend'

type Error
  = { type: 'Api', error: ApiError }
  | { type: 'Other', error: { title: string, message: string }}
  | null

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)
  let error: Error = null

  let ownedTickets: TicketListingViewExpandedItem[] = []
  let userMap: Record<string, UserProfileView> = {}
  let groupMap: Record<string, GroupProfileView> = {}

  try {
    const result = await api.getOwnedTickets()
    if (result.status === 'Success') {
      const { users, groups, payload } = result.payload
      ownedTickets = payload
      userMap = users
      groupMap = groups
    } else {
      console.error(result.payload)
      error = { type: 'Api', error: result.payload }
    }
  } catch (e) {
    console.error(e)
    error = { type: 'Other', error: { title: 'Unexpected error', message: (e as any)?.message || ''}}
  }

  return {
    receivers : await getReceivers(),
    userMap,
    groupMap,
    ownedTickets,
    error,
  }
}

import { getReceivers } from '$lib/mocks/database'
import { requireAuth } from '$lib/utils'
import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { TicketListingViewExpandedItem, GroupProfileView, UserProfileView } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)

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
      // TODO: error handling
      console.error(result.payload.report)
    }
  } catch (error) {
    // TODO: error handling
  }

  return {
    receivers : await getReceivers(),
    userMap,
    groupMap,
    ownedTickets,
  }
}

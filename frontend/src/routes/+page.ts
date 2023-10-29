import { getReceivers, getRequests } from '$lib/mocks/database'
import { requireAuth } from '$lib/utils'
import { ticketDestToMaps } from '$lib/utils/api'
import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { TicketListingViewExpandedItem } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)

  let ownedTickets: TicketListingViewExpandedItem[] = []
  let userMap = new Map<string,string>()
  let groupMap = new Map<string,string>()

  try {
    const result = await api.getOwnedTickets()
    if (result.status === 'Success') {
      ownedTickets = result.payload
      const [users, groups] = await ticketDestToMaps(fetch, ownedTickets.map(t => t.destination))
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

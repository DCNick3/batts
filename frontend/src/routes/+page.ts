import { getReceivers, getRequests } from "$lib/mocks/database"
import { requireAuth } from "$lib/utils"
import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { TicketListingViewExpandedItem } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)

  let ownedTickets: TicketListingViewExpandedItem[] = []
  try {
    const result = await api.getOwnedTickets()
    if (result.status === 'Success') {
      ownedTickets = result.payload
    } else {
      // TODO: error handling
      console.error(result.payload.report)
    }
  } catch (error) {
    // TODO: error handling
  }

  return {
    receivers : await getReceivers(),
    requests: ownedTickets,
  }
}

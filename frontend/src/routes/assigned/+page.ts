import { getAssigneeRequests } from '$lib/mocks/database'
import type { PageLoad } from './$types'
import { requireAuth } from '$lib/utils'
import { Api, type TicketListingViewExpandedItem } from 'backend'
import { ticketDestToMaps } from '$lib/utils/api'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)
  let assignedTickets: TicketListingViewExpandedItem[] = []
  let userMap = new Map<string,string>()
  let groupMap = new Map<string,string>()

  try {
    const result = await api.getAssignedTickets()

    if (result.status === 'Success') {
      assignedTickets = result.payload
      const [users, groups] = await ticketDestToMaps(fetch, assignedTickets.map(t => t.destination))
      userMap = users
      groupMap = groups
    } else {
      // TODO error-handling
      console.error(result.payload.report)
    }
  } catch (error) {
    //TODO: error-handling
    console.error(error)
  }

  return {
    tickets: assignedTickets,
    userMap,
    groupMap,
  }
}

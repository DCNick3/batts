import { getReceivers, getRequests } from "$lib/mocks/database"
import { requireAuth } from "$lib/utils"
import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { TicketListingViewExpandedItem } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)

  let ownedTickets: TicketListingViewExpandedItem[] = []
  let destinations: string[] = []
  try {
    const result = await api.getOwnedTickets()
    if (result.status === 'Success') {
      ownedTickets = result.payload
      const responses = await Promise.all(ownedTickets.map(ticket => {
        if (ticket.destination.type === 'Group') {
          return api.getGroup(ticket.destination.id).then(res => {
            if (res.status === 'Success') {
              return res.payload.title
            } else {
              return 'No-one'
            }
          })
        } else {
          return api.getUserProfile(ticket.destination.id).then(res => {
            if (res.status === 'Success') {
              return res.payload.name
            } else {
              return 'No-one'
            }
          })
        }
      }))
      destinations = responses
    } else {
      // TODO: error handling
      console.error(result.payload.report)
    }
  } catch (error) {
    // TODO: error handling
  }

  return {
    receivers : await getReceivers(),
    requests: ownedTickets.map((ticket, ind) => [ticket, destinations[ind]] as [TicketListingViewExpandedItem, string]),
  }
}

import { getAssigneeRequests } from '$lib/mocks/database'
import type { PageLoad } from './$types'
import { requireAuth } from '$lib/utils'
import { Api, type TicketListingViewExpandedItem } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)
  let assignedTickets: TicketListingViewExpandedItem[] = []
  let destinations: string[] = []

  try {
    const result = await api.getAssignedTickets()

    if (result.status === 'Success') {
      assignedTickets = result.payload
      destinations = await Promise.all(assignedTickets.map(ticket => {
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
    } else {
      // TODO error-handling
      console.error(result.payload.report)
    }
  } catch (error) {
    //TODO: error-handling
    console.error(error)
  }

  return { tickets: assignedTickets.map((ticket, ind) => [ticket, destinations[ind]] as [TicketListingViewExpandedItem, string]) }
}

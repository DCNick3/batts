import { getAssigneeRequests } from '$lib/mocks/database'
import type { PageLoad } from './$types'
import { requireAuth } from '$lib/utils'
import { Api, type TicketListingViewExpandedItem } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)
  let tickets: TicketListingViewExpandedItem[] = []

  try {
    const result = await api.getAssignedTickets()

    if (result.status === 'Success') {
      tickets = result.payload
    } else {
      // TODO error-handling
      console.error(result.payload.report)
    }
  } catch (error) {
    //TODO: error-handling
    console.error(error)
  }

  return { tickets }
}

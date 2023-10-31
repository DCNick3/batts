import type { PageLoad } from './$types'
import { requireAuth } from '$lib/utils'
import { Api } from 'backend'
import type { TicketListingViewExpandedItem, GroupProfileView, UserProfileView } from 'backend'

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  const api = new Api(fetch)
  let assignedTickets: TicketListingViewExpandedItem[] = []
  let userMap: Record<string, UserProfileView> = {}
  let groupMap: Record<string, GroupProfileView> = {}

  try {
    const result = await api.getAssignedTickets()

    if (result.status === 'Success') {
      const { users, groups, payload } = result.payload
      assignedTickets = payload
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

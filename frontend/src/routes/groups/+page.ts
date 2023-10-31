import { Api } from 'backend'
import type { GroupId, GroupView, TicketListingViewExpandedItem, WithGroupsAndUsers } from 'backend'
import type { PageLoad } from './$types'

type TicketData = WithGroupsAndUsers<TicketListingViewExpandedItem[]>

export const load: PageLoad = async ({ fetch, parent }) => {
  const api = new Api(fetch)
  const { userGroups } = await parent()

  let groupTickets = new Map<GroupId, TicketData>()

  try {
    const promises = userGroups.map(grp => {
      return api.getGroupTickets(grp.id).then(result => {
        if (result.status === 'Success') {
          return [grp.id, result.payload]
        } else {
          return null
        }
      })
    })
    const tickets = await Promise.all(promises).then(lst => lst.filter(t => t !== null) as [GroupId, TicketData][])
    tickets.forEach(([id, tick]) => {
      groupTickets.set(id, tick)
    })
  } catch (error) {
    // TODO: error handling
    console.error(error)
  }

  return { groupTickets }
}
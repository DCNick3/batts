import { Api } from 'backend'
import type { ApiError, GroupId, TicketListingViewExpandedItem, WithGroupsAndUsers } from 'backend'
import type { PageLoad } from './$types'

type TicketData = WithGroupsAndUsers<TicketListingViewExpandedItem[]>

type Error
  = { type: 'Api', error: ApiError }
  | { type: 'Other', error: { title: string, message: string }}

export const load: PageLoad = async ({ fetch, parent }) => {
  const api = new Api(fetch)
  const { userGroups } = await parent()

  const groupTickets = new Map<GroupId, TicketData>()
  const errors: Error[] = []

  try {
    const promises = userGroups.map(grp => {
      return api.getGroupTickets(grp.id).then(result => {
        if (result.status === 'Success') {
          const res: { status: 'Success', payload: [string, WithGroupsAndUsers<TicketListingViewExpandedItem[]>]}
            = { status: 'Success', payload: [grp.id, result.payload] }
          return res
        } else {
          return result
        }
      })
    })
    const ticketResults = await Promise.all(promises)
    ticketResults.forEach(res => {
      if (res.status === 'Success') {
        const [id, tick] = res.payload
        groupTickets.set(id, tick)
      } else {
        errors.push({ type: 'Api', error: res.payload })
      }
    })
  } catch (error) {
    console.error(error)
    errors.push({ type: 'Other', error: { title: 'Unexpected error', message: (error as any)?.message || '' }})
  }

  return { groupTickets, errors }
}
import type { PageLoad } from './$types'
import { requireAuth } from '$lib/utils'
import { Api } from 'backend'
import type { TicketListingViewExpandedItem, GroupProfileView, UserProfileView, ApiError } from 'backend'

type Error
  = { type: 'Api', error: ApiError }
  | { type: 'Other', error: { title: string, message: string }}
  | null

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  let error: Error = null

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
      console.error(result.payload.report)
      error = { type: 'Api', error: result.payload }
    }
  } catch (e) {
    console.error(e)
    error = { type: 'Other', error: { title: 'Unexpected error', message: (e as any)?.message || ''}}
  }

  return {
    tickets: assignedTickets,
    userMap,
    groupMap,
    error,
  }
}

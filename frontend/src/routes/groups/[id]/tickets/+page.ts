import { Api } from 'backend'
import type { ApiError, GroupView, TicketListingViewExpandedItem, WithGroupsAndUsers } from 'backend'
import type { PageLoad } from './$types'
import { redirect } from '@sveltejs/kit'

type Error
= { type: 'Api', error: ApiError }
| { type: 'Other', error: { title: string, message: string }}
| null

export const load: PageLoad = async ({ fetch, params, parent }) => {
  const api = new Api(fetch)
  const { user, userGroups } = await parent()

  let error: Error = null

  // If user is not allowed to view tickets of this group,
  // redirect to the group page
  if (user === null || !userGroups.some(group => group.id === params.id)) {
    throw redirect(302, `/groups/${params.id}`)
  }

  let group: null | GroupView = null
  let groupTickets: WithGroupsAndUsers<TicketListingViewExpandedItem[]> = { users: {}, groups: {}, payload: [] }

  try {
    const result = await api.getGroup(params.id)

    if (result.status === 'Success') {
      // NOTE: we actually have a list of users here, we just don't use it yet
      group = result.payload.payload
      const response = await api.getGroupTickets(group.id)
      if (response.status === 'Success') {
        groupTickets = response.payload
      } else {
        console.log(response.payload)
        error = { type: 'Api', error: response.payload }
      }
    } else {
      console.error(result.payload)
      error = { type: 'Api', error: result.payload }
    }
  } catch (e) {
    console.error(e)
    error = { type: 'Other', error: { title: 'Unexpected error', message: (e as any)?.message || ''}}
  }

  return { group, groupTickets, error }
}

import { Api, type GroupView } from 'backend'
import type { PageLoad } from './$types'
import { redirect } from '@sveltejs/kit'

export const load: PageLoad = async ({ fetch, params, parent }) => {
  const api = new Api(fetch)
  const { user, userGroups } = await parent()

  // If user is not allowed to view tickets of this group,
  // redirect to the group page
  if (user === null || !userGroups.some(group => group.id === params.id)) {
    throw redirect(302, `/groups/${params.id}`)
  }

  let group: null | GroupView = null

  try {
    const result = await api.getGroup(params.id)

    if (result.status === 'Success') {
      // NOTE: we actually have a list of users here, we just don't use it yet
      group = result.payload.payload
    } else {
      // TODO: error handling
      console.error(result.payload)
    }
  } catch (error) {
    // TODO: error handling
    console.error(error)
  }

  return { group }
}

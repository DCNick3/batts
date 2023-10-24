import { redirect } from '@sveltejs/kit'
import { Api } from 'backend'
import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch, params, parent }) => {
  const { user } = await parent()
  if (user?.id === params.id) {
    throw redirect(302, '/me')
  }

  const api = new Api(fetch)
  const result = await api.getUserProfile(params.id)
  if (result.status === 'Success') {
    const response = await api.getUserGroups(result.payload.id)
    if (response.status === 'Success') {
      return {
        userProfile: result.payload,
        userGroups: response.payload
      }
    } else {
      // TODO: error handling
      console.error(response.payload)
      return {
        userProfile: result.payload,
        userGroups: []
      }
    }
  } else {
    // TODO: error handling
    console.error(result.payload)
    return { userProfile: null, userGroups: [] }
  }
}

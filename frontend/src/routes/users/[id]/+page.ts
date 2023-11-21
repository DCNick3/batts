import { redirect } from '@sveltejs/kit'
import { Api, type ApiError, type GroupView } from 'backend'
import type { PageLoad } from './$types'

type Error
  = { type: 'Api', error: ApiError }
  | { type: 'Other', error: { title: string, message: string }}
  | null

export const load: PageLoad = async ({ fetch, params, parent }) => {
  const { user } = await parent()
  if (user?.id === params.id) {
    throw redirect(302, '/me')
  }

  let error: Error = null

  const api = new Api(fetch)
  const result = await api.getUserProfile(params.id)
  if (result.status === 'Success') {
    const response = await api.getUserGroups(result.payload.id)
    if (response.status === 'Success') {
      return {
        userProfile: result.payload,
        userGroups: response.payload.payload,
        error
      }
    } else {
      console.error(response.payload)
      error = { type: 'Api', error: response.payload }
      return {
        userProfile: result.payload,
        userGroups: [] as GroupView[],
        error
      }
    }
  } else {
    console.error(result.payload)
    error = { type: 'Api', error: result.payload }
    return { userProfile: null, userGroups: [] as GroupView[], error }
  }
}

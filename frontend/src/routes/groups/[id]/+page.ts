import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { ApiError, GroupView, UserId, UserProfileView } from 'backend'

type Error
  = { type: 'Api', error: ApiError }
  | { type: 'Other', error: { title: string, message: string }}
  | null

// PageLoad<{ groupInfo: GroupView | null, groupUsers: Record<UserId, UserProfileView> | null }>
export const load = async ({ fetch, params }) => {
  const api = new Api(fetch)

  let groupInfo: { view: GroupView, users: Record<UserId, UserProfileView> } | null = null
  let error: Error = null

  try {
    const result = await api.getGroup(params.id)
    if (result.status === 'Success') {
      groupInfo = { view: result.payload.payload, users: result.payload.users }
    } else {
      console.error(result.payload)
      error = { type: 'Api', error: result.payload}
    }
  } catch (e) {
    console.error(e)
    error = { type: 'Other', error: { title: 'Unexpected error', message: (e as any)?.message || '' }}
  }
  return { groupInfo, groupId: params.id, error }
}

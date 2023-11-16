import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { GroupView, UserId, UserProfileView } from 'backend'

// PageLoad<{ groupInfo: GroupView | null, groupUsers: Record<UserId, UserProfileView> | null }>
export const load = async ({ fetch, params }) => {
  const api = new Api(fetch)

  let groupInfo: { view: GroupView, users: Record<UserId, UserProfileView> } | null = null

  try {
    const result = await api.getGroup(params.id)
    if (result.status === 'Success') {
      // const { users, payload: groupInfo } = result.payload
      groupInfo = { view: result.payload.payload, users: result.payload.users }
    } else {
      // TODO handle error
      console.error(result.payload)
    }
  } catch (error) {
    // TODO handle errors
    console.error(error)
  }
  return { groupInfo, groupId: params.id }
}

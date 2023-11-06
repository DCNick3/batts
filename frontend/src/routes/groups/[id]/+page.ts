import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { GroupView, UserId, UserProfileView } from 'backend'

export const load: PageLoad<{ groupInfo: GroupView | null, groupUsers: Record<UserId, UserProfileView> | null }> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  let groupInfo: null | GroupView = null
  let groupUsers: null | Record<UserId, UserProfileView> = null

  try {
    const result = await api.getGroup(params.id)
    if (result.status === 'Success') {
      // const { users, payload: groupInfo } = result.payload
      groupInfo = result.payload.payload
      groupUsers = result.payload.users
    } else {
      // TODO handle error
      console.error(result.payload)
    }
  } catch (error) {
    // TODO handle errors
    console.error(error)
  }
  return { groupInfo, groupUsers, groupId: params.id }
}

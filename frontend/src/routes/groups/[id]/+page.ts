import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { GroupView, UserId, UserProfileView } from 'backend'

export const load: PageLoad<{ groupInfo: GroupView | null, users: Record<UserId, UserProfileView> | null }> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  try {
    const result = await api.getGroup(params.id)
    if (result.status === 'Success') {

      const { users, payload: groupInfo } = result.payload;

      return { groupInfo, users }
    } else {
      // TODO handle error
      console.error(result.payload)
      return { groupInfo: null, users: null }
    }
  } catch (error) {
    // TODO handle errors
    console.error(error)
    return { groupInfo: null, users: null }
  }
}

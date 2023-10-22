import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { ApiError, GroupViewContent } from 'backend'

export const load: PageLoad<{ groupInfo: GroupViewContent | null }> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  try {
    const result = await api.getGroup(params.id)
    if (result.status === 'Success') {
      const userIds = new Set<string>()
      result.payload.members.forEach(userId => {
          userIds.add(userId)
      })
      const users = new Map<string,string>()

      const responses = await Promise.all(Array.from(userIds).map(id => api.getUserProfile(id)))

      responses.forEach(res => {
        if (res.status === "Success") {
          users.set(res.payload.id, res.payload.name) 
        } else {
          console.error(res.payload.report)
        }
      })
      return { groupInfo: result.payload, users }
    } else {
      // TODO handle error
      console.error(result.payload)
      return { groupInfo: null }
    }
  } catch (error) {
    // TODO handle errors
    console.error(error)
    return { groupInfo: null }
  }
}

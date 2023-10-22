import type { PageLoad } from './$types'
import { Api } from 'backend'
import type { ApiError, GroupViewContent } from 'backend'

export const load: PageLoad<{ groupInfo: GroupViewContent | null }> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  try {
    const result = await api.getGroup(params.id)
    if (result.status === 'Success') {
      return { groupInfo: result.payload }
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

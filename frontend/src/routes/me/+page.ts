import { Api } from 'backend'
import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch, parent }) => {
  const api = new Api(fetch)
  const { user } = await parent()

  if (user !== null) {
    const result = await api.getUserGroups(user.id)
    if (result.status === 'Success') {
      return { groups: result.payload }
    } else {
      // TODO error handling
      console.error(result.payload)
      return { groups: [] }
    }
  } else {
    return { groups: [] }
  }
}

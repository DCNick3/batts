import { Api, type GroupView, type WithUsers } from 'backend'
import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch, parent }) => {
  const api = new Api(fetch)
  const { user, userGroups } = await parent()
}

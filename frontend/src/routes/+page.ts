import { getReceivers, getRequests } from "$lib/mocks/database"
import type { PageLoad } from './$types'
import { Api } from 'backend'

export const load: PageLoad = async ({ fetch }) => {
  const api = new Api(fetch)

  return {
    receivers : await getReceivers(),
    requests: await getRequests(),
  }
}

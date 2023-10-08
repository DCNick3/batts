import { getAssigneeRequests } from "$lib/mocks/database"
import type { PageLoad } from './$types'

export const load: PageLoad = async ({ fetch }) => {
  return {
    requests: await getAssigneeRequests(),
  }
}

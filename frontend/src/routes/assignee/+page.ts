import { getAssigneeRequests } from "$lib/mocks/database"
import type { PageLoad } from './$types'

export const load: PageLoad = async () => {
  return {
    requests: await getAssigneeRequests(),
  }
}

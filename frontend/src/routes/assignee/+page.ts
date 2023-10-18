import { getAssigneeRequests } from "$lib/mocks/database"
import type { PageLoad } from './$types'
import { requireAuth } from "$lib/utils"

export const load: PageLoad = async ({ fetch, parent }) => {
  await requireAuth(parent)

  return {
    requests: await getAssigneeRequests(),
  }
}

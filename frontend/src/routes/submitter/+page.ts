import { getReceivers, getRequests, submitRequest } from "$lib/mocks/database"
import type { PageLoad } from './$types'

export const load: PageLoad = async () => {
  return {
    receivers : await getReceivers(),
    requests: await getRequests(),
  }
}

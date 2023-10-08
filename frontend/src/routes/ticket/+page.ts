import { getTicketData } from '$lib/mocks/database'
import type { PageLoad } from './$types'

export const load: PageLoad = async () => {
  return await getTicketData()
}

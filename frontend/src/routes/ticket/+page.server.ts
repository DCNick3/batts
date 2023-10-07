import { getTicketData } from '$lib/server/database'

export function load() {
  return getTicketData()
}

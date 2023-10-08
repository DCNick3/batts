import { getTicketData } from '$lib/mocks/database'
import type { PageLoad } from './$types'
import { Api } from '$lib/api'
import type { TicketView } from 'backend/bindings/TicketView'

export const load: PageLoad<TicketView> = async ({ fetch, params }) => {
  const api = new Api(fetch)
  const result = await api.getTicket(params.id)

  if (result.status === "Error") throw Error(result.payload.report)
  return result.payload as TicketView
}

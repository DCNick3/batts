import type { PageLoad } from './$types'
import { Api } from 'backend'

export const load: PageLoad = async ({ fetch, params }) => {
  const api = new Api(fetch)
  const result = await api.getTicket(params.id)

  if (result.status === "Error") throw Error(result.payload.report)
  return result.payload
}

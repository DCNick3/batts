import type { PageLoad } from './$types'
import { Api } from 'backend'

export const load: PageLoad = async ({ fetch, params }) => {
  const api = new Api(fetch)
  const result = await api.getTicket(params.id)

  if (result.status === "Success") {
    const userIds = new Set<string>()
    result.payload.timeline.forEach(item => {
      if (item.content.type === "Message") {
        userIds.add(item.content.from)
      }
    })
    const users = new Map<string,string>()

    const responses = await Promise.all(Array.from(userIds).map(id => api.getUserProfile(id)))

    responses.forEach(res => {
      if (res.status === "Success") {
        users.set(res.payload.id, res.payload.name) 
      } else {
        console.error(res.payload.report)
      }
    })

    return { users, ticketId: params.id, ...result }
  }

  return { ticketId: params.id, ...result }
}

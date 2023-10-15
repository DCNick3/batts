import type { PageLoad } from './$types'
import type { TicketView, ApiError } from 'backend'
import { Api } from 'backend'

/*
  TypeScript is when you put lots of `as` hack,
  yet still believe you have type safety
*/
type Hack = { status: 'ConnectionError', ticketId: string }
type Data
  = { status: 'Success', payload: TicketView, users: Map<string, string>, ticketId: string }
  | { status: 'Error', payload: ApiError, ticketId: string }
  | Hack

export const load: PageLoad<Data> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  try {
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
  } catch (error) {
    return { ticketId: params.id, status: 'ConnectionError'} as Hack
  }
}

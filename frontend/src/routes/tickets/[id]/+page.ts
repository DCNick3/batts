import type { PageLoad } from './$types'
import type { TicketViewContent, ApiError } from 'backend'
import { Api } from 'backend'

/*
  TypeScript is when you put lots of `as` hack,
  yet still believe you have type safety
*/
type Hack = { status: 'ConnectionError', ticketId: string }
type Data
  = { status: 'Success', payload: TicketViewContent, users: Map<string, string>, ticketId: string }
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
      if (result.payload.assignee) {
        userIds.add(result.payload.assignee)
      }
      const users = new Map<string,string>()

      const responses = await Promise.all(Array.from(userIds).map(id => api.getUserProfile(id)))

      responses.forEach(res => {
        if (res.status === "Success") {
          users.set(res.payload.id, res.payload.name) 
        } else {
          console.error(res.payload.report)
        }
      })

      const editPermissions = new Set<string>()
      let destinationField: string = ''
      const destination = result.payload.destination
      // @ts-ignore
      if (destination.Group) {
        // @ts-ignore
        const res = await api.getGroup(destination.Group)
        if (res.status === 'Success') {
          res.payload.members.forEach(m => { editPermissions.add(m) })
          destinationField = res.payload.title
        } else {
          // TODO handle group info load failure
          console.error(res.payload)
        }
      } else {
        // @ts-ignore
        editPermissions.add(destination.User)
        // @ts-ignore
        const res = await api.getUserProfile(destination.User)
        if (res.status === 'Success') {
          destinationField = res.payload.name
        }
      }
      if (result.payload.assignee) {
        editPermissions.add(result.payload.assignee)
      }

      return { users, ticketId: params.id, editPermissions, destinationField, ...result }
    }

    return { ticketId: params.id, ...result }
  } catch (error) {
    return { ticketId: params.id, status: 'ConnectionError'} as Hack
  }
}

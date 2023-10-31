import type { PageLoad } from './$types'
import type {TicketView, ApiError, UserProfileView, UserId, TicketId} from 'backend'
import { Api } from 'backend'

/*
  TypeScript is when you put lots of `as` hack,
  yet still believe you have type safety
*/
type Hack = { status: 'ConnectionError', ticketId: TicketId }
type Data
  = { status: 'Success', payload: TicketView, users: Record<UserId, UserProfileView>, ticketId: TicketId, ticket: TicketView }
  | { status: 'Error', payload: ApiError, ticketId: TicketId }
  | Hack

export const load: PageLoad<Data> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  try {
    const result = await api.getTicket(params.id)

    if (result.status === "Success") {
      const { users, payload: ticket } = result.payload

      const groupMembers = new Map<string,string>()

      const editPermissions = new Set<string>()
      let destinationField: string = ''
      const destination = ticket.destination
      if (destination.type === 'Group') {
        const res = await api.getGroup(destination.id)
        if (res.status === 'Success') {
          const { users, payload: group } = res.payload

          group.members.forEach(m => {
            editPermissions.add(m);
            groupMembers.set(m, users[m].name);
          })
        } else {
          // TODO handle group info load failure
          console.error(res.payload)
        }
      } else {
        editPermissions.add(destination.id)
        const res = await api.getUserProfile(destination.id)
        if (res.status === 'Success') {
          destinationField = res.payload.name
        }
      }
      if (ticket.assignee) {
        editPermissions.add(ticket.assignee)
      }

      return { users, ticketId: params.id, editPermissions, destinationField, groupMembers, ticket }
    }

    return { ticketId: params.id, status: "Error" }
  } catch (error) {
    return { ticketId: params.id, status: 'ConnectionError'} as Hack
  }
}

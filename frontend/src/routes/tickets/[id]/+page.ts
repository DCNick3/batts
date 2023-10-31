import type { PageLoad } from './$types'
import type {TicketView, ApiError, UserProfileView, UserId, TicketId, GroupId, GroupProfileView} from 'backend'
import { Api } from 'backend'

/*
  TypeScript is when you put lots of `as` hacks,
  yet still believe you have type safety
*/
type ConErr = { status: 'ConnectionError', ticketId: TicketId }
type Err = { status: 'Error', payload: ApiError, ticketId: TicketId }
type Succ = {
  status: 'Success',
  editPermissions: Set<string>,
  users: Record<UserId, UserProfileView>,
  groups: Record<GroupId, GroupProfileView>,
  ticketId: TicketId,
  ticket: TicketView,
}
type Data
  = Succ
  | Err
  | ConErr

export const load: PageLoad<Data> = async ({ fetch, params }) => {
  const api = new Api(fetch)

  try {
    const result = await api.getTicket(params.id)

    if (result.status === "Success") {
      const { users, groups, payload: ticket } = result.payload

      const editPermissions = new Set<string>()
      const destination = ticket.destination
      if (destination.type === 'Group') {
        const res = await api.getGroup(destination.id)
        if (res.status === 'Success') {
          const { payload: group } = res.payload

          group.members.forEach(m => {
            editPermissions.add(m);
          })
        } else {
          // TODO handle group info load failure
          console.error(res.payload)
        }
      } else {
        editPermissions.add(destination.id)
      }
      if (ticket.assignee) {
        editPermissions.add(ticket.assignee)
      }

      return { status: 'Success', users, ticketId: params.id, editPermissions, groups, ticket } as Succ
    } else {
      return { ticketId: params.id, status: 'Error', payload: result.payload } as Err
    }
  } catch (error) {
    return { ticketId: params.id, status: 'ConnectionError'} as ConErr
  }
}

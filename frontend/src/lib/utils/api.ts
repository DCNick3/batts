import { Api } from 'backend'
import type { ApiResult, GroupView, TicketDestination, UserProfileView } from 'backend'

export const ticketDestToMaps = async (f: typeof fetch, destinations: TicketDestination[]) => {
  const api = new Api(f)

  const groups = new Map<string,string>()
  const users = new Map<string,string>()

  const userP: Promise<ApiResult<UserProfileView>>[] = []
  const groupP: Promise<ApiResult<GroupView>>[] = []
  destinations.forEach(dest => {
    if (dest.type === 'Group') {
      groupP.push(api.getGroup(dest.id))
    } else {
      userP.push(api.getUserProfile(dest.id))
    }
  })

  try {
    const usrResults = await Promise.all(userP)
    usrResults.forEach(res => {
      if (res.status === 'Success') {
        users.set(res.payload.id, res.payload.name)
      } else {
        // TODO: error handling
        console.error(res.payload)
      }
    })

    const grpResults = await Promise.all(groupP)
    grpResults.forEach(res => {
      if (res.status === 'Success') {
        groups.set(res.payload.id, res.payload.title)
      } else {
        // TODO: error handling
        console.error(res.payload)
      }
    })
  } catch (error) {
    // TODO: error handling
    console.error(error)
  }

  return [users, groups]
}

import { getAssigneeRequests } from "$lib/server/database"

export function load() {
  return {
    requests: getAssigneeRequests(),
  }
}

import { getRequests } from "$lib/server/database"

export function load() {
  return {
    requests: getRequests(),
  }
}

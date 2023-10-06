import { getRequests } from "$lib/server/data"

export function load() {
  return {
    requests: getRequests(),
  }
}

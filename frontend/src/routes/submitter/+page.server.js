import { getReceivers, getRequests } from "$lib/server/data"

export function load() {
  return {
    receivers : getReceivers(),
    requests: getRequests(),
  }
}

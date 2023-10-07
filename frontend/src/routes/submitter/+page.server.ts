import { getReceivers, getRequests, submitRequest } from "$lib/server/database"

export function load() {
  return {
    receivers : getReceivers(),
    requests: getRequests(),
  }
}

export const actions = {
  submit: async ({ cookies, request }) => {
    const data = request.formData()
    submitRequest(data)
    // TODO
  }
}
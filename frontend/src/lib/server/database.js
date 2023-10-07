// TODO: use actual db
export function getReceivers() {
  return [
    { name : "IT Department", id : "it" },
    { name : "Dorm Manager", id : "dorm" },
  ]
}

export function getRequests() {
  return [
    { receiver: "Dorm Manager", topic: "Broken chair", status: "Pending", up: false },
    { receiver: "IT Department", topic: "No internet", status: "Pending", up: true },
    { receiver: "Dorm Manager", topic: "Doorknob", status: "In process", up: false },
    { receiver: "Dorm Manager", topic: "Broken bulb", status: "Fixed", up: false },
    { receiver: "IT Department", topic: "Dashboard broken", status: "Fixed", up: true },
  ]
}

// @ts-ignore
export function submitRequest(request) {
  // TODO
}
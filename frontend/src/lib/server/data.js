// TODO: use actual db
export function getReceivers() {
  return [
    { name : "IT Department", id : "it" },
    { name : "Dorm Manager", id : "dorm" },
  ]
}

export function getRequests() {
  return [
    { receiver: "Dorm Manager", topic: "Broken chair", status: "Pending" },
    { receiver: "IT Department", topic: "No internet", status: "Pending" },
    { receiver: "Dorm Manager", topic: "Doorknob", status: "In process" },
    { receiver: "Dorm Manager", topic: "Broken bulb", status: "Fixed" },
    { receiver: "IT Department", topic: "Dashboard broken", status: "Fixed" },
  ]
}
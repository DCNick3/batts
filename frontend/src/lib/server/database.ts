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

export function getAssigneeRequests() {
  return [
    { sender: "Vasiliy Terkin", topic: "Broken chair", status: "Pending", up: false },
    { sender: "Mikhail Olokin", topic: "No internet", status: "Pending", up: true },
    { sender: "Rebeca", topic: "Doorknob", status: "In process", up: false },
    { sender: "Bulb breaker", topic: "Broken bulb", status: "Fixed", up: false },
    { sender: "Dasha Boardina", topic: "Dashboard broken", status: "Fixed", up: true },
  ]
}

export type Message
  = { type: 'message', text: string, is_sender: boolean, date: string }
  | { type: 'notification', text: string, date: string }
  
type TicketData
  = { topic: string, sender: string, receiver: string, status: string, messages: Message[] }

export function getTicketData(): TicketData {
  return {
    topic: "Dashboard broken",
    sender: "Dasha Boardina",
    receiver: "IT Department",
    status: "In process",
    messages: [
      { type: "message", text: "Hello! When will you fix it?", is_sender: true, date: "20 February" },
      { type: "message", text: "Good afternoon! Could you please provide more details on the problem?", is_sender: false, date: "10 February" },
      { type: "notification", text: 'Status changed to "In process"', date: '10 February' },
      { type: "message", text: "Hello! Dashboard broken :(", is_sender: true, date: "6 February" },
    ]
  }
}

// @ts-ignore
export function submitRequest(request) {
  // TODO
}
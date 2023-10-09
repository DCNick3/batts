// TODO: use actual db
export async function getReceivers() {
  return [
    { name : "IT Department", id : "it" },
    { name : "Dorm Manager", id : "dorm" },
  ]
}

export async function getRequests() {
  return [
    { id: "a", receiver: "Dorm Manager", topic: "Broken chair", status: "Pending", up: false },
    { id: "b", receiver: "IT Department", topic: "No internet", status: "Pending", up: true },
    { id: "c", receiver: "Dorm Manager", topic: "Doorknob", status: "In process", up: false },
    { id: "d", receiver: "Dorm Manager", topic: "Broken bulb", status: "Fixed", up: false },
    { id: "e", receiver: "IT Department", topic: "Dashboard broken", status: "Fixed", up: true },
  ]
}

export async function getAssigneeRequests() {
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

export async function getTicketData(): Promise<TicketData> {
  return {
    topic: "Dashboard broken",
    sender: "Dasha Boardina",
    receiver: "IT Department",
    status: "In process",
    messages: [
      { type: "message", text: "Hello! Dashboard broken :(", is_sender: true, date: "6 February" },
      { type: "notification", text: 'Status changed to "In process"', date: '10 February' },
      { type: "message", text: "Good afternoon! Could you please provide more details on the problem?", is_sender: false, date: "10 February" },
      { type: "message", text: "Hello! When will you fix it?", is_sender: true, date: "20 February" },
    ]
  }
}

export type UserInfo = {
  id: string,
  name: string,
  identities: {
    telegram: {
      id: number,
      first_name: string,
      last_name: string,
      username: null,
      photo_url: string | null
    } | null,
    university: null
  }
}

export async function getMe(): Promise<UserInfo> {
  return {
    id: "FDUeanyKADQEpyrydYn7XB",
    name: "Dahsa Boardina",
    identities: {
      telegram: {
        id: 123456,
        first_name: "Dasha",
        last_name: "Boardina",
        username: null,
        "photo_url": null
      },
      university: null
    }
  }
}

// @ts-ignore
export function submitRequest(request) {
  // TODO
}
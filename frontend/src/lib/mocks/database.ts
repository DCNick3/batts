import type { TicketDestination } from 'backend'

export async function getReceivers() {
  return [
    { name : "IT Department", id : "ItDepartment" },
    { name : "Dorm Manager", id : "DormManager" },
  ] as { name: string, id: TicketDestination }[]
}

export async function getRequests() {
  return [
    { id: "F2VaZtXgAKgxJncCbMbX9V", receiver: "Dorm Manager", topic: "Broken chair", status: "Pending", up: false },
    { id: "H3NbS5NeKY33AMr6Pvtw6H", receiver: "IT Department", topic: "No internet", status: "Pending", up: true },
    { id: "XYF1Ur6Z4oeVBioYtW62nF", receiver: "Dorm Manager", topic: "Doorknob", status: "In process", up: false },
    { id: "M1A5QazKGRUNoTqraWxYou", receiver: "Dorm Manager", topic: "Broken bulb", status: "Fixed", up: false },
    { id: "BJytpHn3GssUW24WJJkrPg", receiver: "IT Department", topic: "Dashboard broken", status: "Fixed", up: true },
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
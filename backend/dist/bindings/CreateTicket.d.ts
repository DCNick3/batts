import type { TicketDestination } from "./TicketDestination";
export interface CreateTicket {
    destination: TicketDestination;
    title: string;
    body: string;
}

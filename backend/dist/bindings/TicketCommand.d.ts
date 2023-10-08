import type { CreateTicket } from "./CreateTicket";
import type { SendTicketMessage } from "./SendTicketMessage";
import type { TicketStatus } from "./TicketStatus";
export type TicketCommand = {
    "type": "Create";
} & CreateTicket | {
    "type": "SendTicketMessage";
} & SendTicketMessage | {
    "type": "ChangeStatus";
} & TicketStatus;

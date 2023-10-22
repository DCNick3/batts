import type { ChangeAssignee } from "./ChangeAssignee";
import type { ChangeStatus } from "./ChangeStatus";
import type { CreateTicket } from "./CreateTicket";
import type { SendTicketMessage } from "./SendTicketMessage";
export type TicketCommand = {
    "type": "Create";
} & CreateTicket | {
    "type": "SendTicketMessage";
} & SendTicketMessage | {
    "type": "ChangeStatus";
} & ChangeStatus | {
    "type": "ChangeAssignee";
} & ChangeAssignee;

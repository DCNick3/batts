import type { ChangeAssignee } from "./ChangeAssignee";
import type { ChangeStatus } from "./ChangeStatus";
import type { SendTicketMessage } from "./SendTicketMessage";
export type UpdateTicket = {
    "type": "SendTicketMessage";
} & SendTicketMessage | {
    "type": "ChangeStatus";
} & ChangeStatus | {
    "type": "ChangeAssignee";
} & ChangeAssignee;

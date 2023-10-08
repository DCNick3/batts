import type { TicketStatus } from "./TicketStatus";
import type { UserId } from "./UserId";
export type TicketTimelineItemContent = {
    "type": "Message";
    from: UserId;
    text: string;
} | {
    "type": "StatusChange";
    old: TicketStatus;
    new: TicketStatus;
};

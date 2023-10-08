import type { TicketId } from "./TicketId";
import type { TicketStatus } from "./TicketStatus";
import type { TicketTimelineItem } from "./TicketTimelineItem";
import type { UserId } from "./UserId";
export interface TicketView {
    id: TicketId;
    owner: UserId;
    title: string;
    status: TicketStatus;
    timeline: Array<TicketTimelineItem>;
}

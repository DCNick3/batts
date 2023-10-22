import type { TicketDestination } from "./TicketDestination";
import type { TicketId } from "./TicketId";
import type { TicketStatus } from "./TicketStatus";
import type { TicketTimelineItem } from "./TicketTimelineItem";
import type { UserId } from "./UserId";
export interface TicketViewContent {
    id: TicketId;
    destination: TicketDestination;
    owner: UserId;
    assignee: UserId | null;
    title: string;
    status: TicketStatus;
    timeline: Array<TicketTimelineItem>;
}

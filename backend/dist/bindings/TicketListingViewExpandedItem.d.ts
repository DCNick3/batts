import type { TicketDestination } from "./TicketDestination";
import type { TicketId } from "./TicketId";
import type { TicketStatus } from "./TicketStatus";
import type { UserId } from "./UserId";
export interface TicketListingViewExpandedItem {
    id: TicketId;
    destination: TicketDestination;
    owner: UserId;
    assignee: UserId | null;
    title: string;
    status: TicketStatus;
    latest_update: string;
}

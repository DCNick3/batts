import type { GroupId } from "./GroupId";
import type { UserId } from "./UserId";
export type TicketDestination = {
    "type": "User";
    "id": UserId;
} | {
    "type": "Group";
    "id": GroupId;
};

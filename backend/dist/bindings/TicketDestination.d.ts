import type { GroupId } from "./GroupId";
import type { UserId } from "./UserId";
export type TicketDestination = {
    "User": UserId;
} | {
    "Group": GroupId;
};

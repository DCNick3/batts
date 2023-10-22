import type { GroupId } from "./GroupId";
import type { UserId } from "./UserId";
export interface GroupViewContent {
    id: GroupId;
    title: string;
    members: Array<UserId>;
}

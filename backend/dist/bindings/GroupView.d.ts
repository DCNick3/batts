import type { GroupId } from "./GroupId";
import type { UserId } from "./UserId";
export interface GroupView {
    id: GroupId;
    title: string;
    members: Array<UserId>;
}

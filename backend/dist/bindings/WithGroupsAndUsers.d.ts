import type { GroupId } from "./GroupId";
import type { GroupProfileView } from "./GroupProfileView";
import type { UserId } from "./UserId";
import type { UserProfileView } from "./UserProfileView";
export interface WithGroupsAndUsers<T> {
    groups: Record<GroupId, GroupProfileView>;
    users: Record<UserId, UserProfileView>;
    payload: T;
}

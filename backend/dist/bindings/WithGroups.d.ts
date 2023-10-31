import type { GroupId } from "./GroupId";
import type { GroupProfileView } from "./GroupProfileView";
export interface WithGroups<T> {
    groups: Record<GroupId, GroupProfileView>;
    payload: T;
}

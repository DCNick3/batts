import type { UserId } from "./UserId";
import type { UserProfileView } from "./UserProfileView";
export interface WithUsers<T> {
    users: Record<UserId, UserProfileView>;
    payload: T;
}

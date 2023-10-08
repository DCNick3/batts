import type { UserId } from "./UserId";
import type { UserIdentities } from "./UserIdentities";
export interface UserView {
    id: UserId;
    name: string;
    identities: UserIdentities;
}

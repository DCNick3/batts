import type { AddGroupMember } from "./AddGroupMember";
import type { ChangeGroupTitle } from "./ChangeGroupTitle";
import type { RemoveGroupMember } from "./RemoveGroupMember";
export type UpdateGroup = {
    "type": "AddMember";
} & AddGroupMember | {
    "type": "RemoveMember";
} & RemoveGroupMember | {
    "type": "ChangeTitle";
} & ChangeGroupTitle;

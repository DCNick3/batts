import type { AddGroupMember } from "./AddGroupMember";
export type UpdateGroup = {
    "type": "AddMember";
} & AddGroupMember;

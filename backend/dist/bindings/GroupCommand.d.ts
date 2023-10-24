import type { AddGroupMember } from "./AddGroupMember";
export type GroupCommand = {
    "type": "AddMember";
} & AddGroupMember;

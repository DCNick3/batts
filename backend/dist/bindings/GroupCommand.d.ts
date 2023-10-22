import type { AddGroupMember } from "./AddGroupMember";
import type { CreateGroup } from "./CreateGroup";
export type GroupCommand = {
    "type": "Create";
} & CreateGroup | {
    "type": "AddMember";
} & AddGroupMember;

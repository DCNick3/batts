import type { ExternalUserProfile } from "./ExternalUserProfile";
export type UserCommand = {
    "type": "Create";
    profile: ExternalUserProfile;
} | {
    "type": "AddIdentity";
    profile: ExternalUserProfile;
};

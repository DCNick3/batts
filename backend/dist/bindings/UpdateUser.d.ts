import type { ExternalUserProfile } from "./ExternalUserProfile";
export type UpdateUser = {
    "type": "AddIdentity";
    profile: ExternalUserProfile;
};

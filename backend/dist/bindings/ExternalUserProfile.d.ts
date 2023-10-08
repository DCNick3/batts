import type { TelegramProfile } from "./TelegramProfile";
import type { UniversityProfile } from "./UniversityProfile";
export type ExternalUserProfile = {
    "type": "Telegram";
} & TelegramProfile | {
    "type": "University";
} & UniversityProfile;

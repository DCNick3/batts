import type { TelegramProfile } from "./TelegramProfile";
import type { UniversityProfile } from "./UniversityProfile";
export type UserProfile = {
    "type": "Telegram";
} & TelegramProfile | {
    "type": "University";
} & UniversityProfile;

import type { TelegramProfile } from "./TelegramProfile";
import type { UniversityProfile } from "./UniversityProfile";
export interface UserIdentities {
    telegram: TelegramProfile | null;
    university: UniversityProfile | null;
}

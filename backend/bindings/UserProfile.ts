// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { TelegramProfile } from "./TelegramProfile";
import type { UniversityProfile } from "./UniversityProfile";

export type UserProfile = { "type": "Telegram" } & TelegramProfile | { "type": "University" } & UniversityProfile;
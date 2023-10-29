export interface TelegramProfile {
    id: number;
    first_name: string;
    last_name: string | null;
    username: string | null;
    photo_url: string | null;
}

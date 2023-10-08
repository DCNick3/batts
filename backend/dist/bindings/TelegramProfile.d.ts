export interface TelegramProfile {
    id: number;
    first_name: string;
    last_name: string;
    username: string | null;
    photo_url: string | null;
}

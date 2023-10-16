export interface TelegramLoginData {
    id: number;
    first_name: string;
    last_name: string;
    username: string | null;
    photo_url: string | null;
    auth_date: number;
    hash: string;
}

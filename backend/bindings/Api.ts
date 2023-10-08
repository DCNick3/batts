import type {
    ApiResult,
    FetchFn,
    UserId,
    UserCommand,
    UserView,
    UserProfileView,
    ExternalUserProfile,
    TicketId,
    CreateTicket,
    TicketCommand,
    TicketView
} from "../";

// import { toDates, toDatesByArray } from 'ts-transformer-dates';

import {v4 as uuidv4, parse as parseUuid} from 'uuid'
import bs58 from "bs58";

export function generateId(): string {
    const uuid = parseUuid(uuidv4());
    return bs58.encode(uuid);
}

export class Api {
    constructor(private fetch: FetchFn) {
    }

    async #sendCommand(url: string, command: { [key: string]: any; }): Promise<ApiResult<null>> {
        const res = await this.fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(command),
        });
        return await res.json();
    }

    async #get<T extends object>(url: string): Promise<ApiResult<T>> {
        const res = await this.fetch(url);
        return await res.json();
    }

    async internalCreateUser(id: UserId, profile: ExternalUserProfile): Promise<ApiResult<null>> {
        let command: UserCommand = {type: "Create", profile};
        return await this.#sendCommand(`/api/users/${id}`, command);
    }

    async internalFakeLogin(userId: UserId): Promise<ApiResult<null>> {
        const res = await this.fetch(`/api/fake-login/${userId}`, {
            method: 'POST',
        });
        return await res.json();
    }

    async getMe(): Promise<ApiResult<UserView>> {
        return await this.#get('/api/users/me');
    }

    async getUserProfile(id: UserId): Promise<ApiResult<UserProfileView>> {
        return await this.#get(`/api/users/${id}/profile`);
    }

    async createTicket(id: TicketId, creation: CreateTicket): Promise<ApiResult<null>> {
        let command: TicketCommand = {type: "Create", ...creation};
        return await this.#sendCommand(`/api/tickets/${id}`, command);
    }

    async getTicket(id: TicketId): Promise<ApiResult<TicketView>> {
        return await this.#get(`/api/tickets/${id}`);
    }
}

export default Api;
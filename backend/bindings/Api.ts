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
} from "@";

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
        const res = await this.fetch('/api/users/me');
        return await res.json();
    }

    async getUserProfile(id: UserId): Promise<ApiResult<UserProfileView>> {
        const res = await this.fetch(`/api/users/${id}/profile`);
        return await res.json();
    }

    async createTicket(id: TicketId, creation: CreateTicket): Promise<ApiResult<null>> {
        let command: TicketCommand = {type: "Create", ...creation};
        return await this.#sendCommand(`/api/tickets/${id}`, command);
    }

    async getTicket(id: TicketId): Promise<ApiResult<TicketView>> {
        const res = await this.fetch(`/api/tickets/${id}`);
        return await res.json();
    }
}

export default Api;
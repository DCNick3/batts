import type { ApiResult } from './ApiResult';

import type { UserId } from "./UserId";
import type { UserView } from './UserView';
import type { UserProfileView } from "./UserProfileView";

import type { TicketId } from "./TicketId";
import type { CreateTicket } from "./CreateTicket";
import type { TicketCommand } from "./TicketCommand";
import type { TicketView } from "./TicketView";

type FetchFn = typeof fetch;

export class Api {
    constructor(private fetch: FetchFn) {}

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

    async getMe(): Promise<ApiResult<UserView>> {
        const res = await this.fetch('/api/users/me');
        return await res.json();
    }

    async getUserProfile(id: UserId): Promise<ApiResult<UserProfileView>> {
        const res = await this.fetch(`/api/users/${id}/profile`);
        return await res.json();
    }

    async createTicket(id: TicketId, creation: CreateTicket): Promise<ApiResult<null>> {
        let command: TicketCommand = { "type": "Create", ...creation };
        return await this.#sendCommand(`/api/ticket/${id}`, command);
    }

    async getTicket(id: TicketId): Promise<ApiResult<TicketView>> {
        const res = await this.fetch(`/api/ticket/${id}`);
        return await res.json();
    }
}
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
    SendTicketMessage,
    TicketCommand,
    TicketViewContent,
    TicketListingViewExpandedItem,
    TicketStatus,
    TelegramLoginData,
    CreateGroup,
    GroupViewContent,
    GroupId,
    GroupCommand,
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

    async #sendCreateCommand(url: string, command: { [key: string]: any; }): Promise<ApiResult<null>> {
        const res = await this.fetch(url, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(command),
        });
        return await res.json();
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

    async telegramLogin(data: TelegramLoginData): Promise<ApiResult<null>> {
        return await this.#sendCommand(`/api/login/telegram`, data);
    }

    async getMe(): Promise<ApiResult<UserView>> {
        return await this.#get('/api/users/me');
    }

    async getUserProfile(id: UserId): Promise<ApiResult<UserProfileView>> {
        return await this.#get(`/api/users/${id}/profile`);
    }

    async getUserGroups(id: UserId): Promise<ApiResult<GroupViewContent[]>> {
        return await this.#get(`/api/users/${id}/groups`);
    }

    async createGroup(id: GroupId, creation: CreateGroup): Promise<ApiResult<null>> {
        return await this.#sendCreateCommand(`/api/groups/${id}`, creation);
    }

    async getGroupTickets(id: GroupId): Promise<ApiResult<Array<TicketListingViewExpandedItem>>> {
        return await this.#get(`/api/groups/${id}/tickets`);
    }

    async getGroup(id: GroupId): Promise<ApiResult<GroupViewContent>> {
        return await this.#get(`/api/groups/${id}`);
    }

    async addGroupMember(id: GroupId, new_member: UserId): Promise<ApiResult<null>> {
        let command: GroupCommand = {type: "AddMember", new_member};
        return await this.#sendCommand(`/api/groups/${id}`, command);
    }

    async createTicket(id: TicketId, creation: CreateTicket): Promise<ApiResult<null>> {
        let command: TicketCommand = {type: "Create", ...creation};
        return await this.#sendCommand(`/api/tickets/${id}`, command);
    }

    async getTicket(id: TicketId): Promise<ApiResult<TicketViewContent>> {
        return await this.#get(`/api/tickets/${id}`);
    }

    async getOwnedTickets(): Promise<ApiResult<Array<TicketListingViewExpandedItem>>> {
        return await this.#get(`/api/tickets/owned`);
    }

    async getAssignedTickets(): Promise<ApiResult<Array<TicketListingViewExpandedItem>>> {
        return await this.#get(`/api/tickets/assigned`);
    }

    async sendTicketMessage(id: TicketId, message: SendTicketMessage): Promise<ApiResult<null>> {
        let command: TicketCommand = {type: "SendTicketMessage", ...message};
        return await this.#sendCommand(`/api/tickets/${id}`, command);
    }

    async changeTicketStatus(id: TicketId, new_status: TicketStatus): Promise<ApiResult<null>> {
        let command: TicketCommand = {type: "ChangeStatus", new_status};
        return await this.#sendCommand(`/api/tickets/${id}`, command);
    }

    async changeTicketAssignee(id: TicketId, new_assignee: UserId | null): Promise<ApiResult<null>> {
        let command: TicketCommand = {type: "ChangeAssignee", new_assignee};
        return await this.#sendCommand(`/api/tickets/${id}`, command);
    }
}

export default Api;
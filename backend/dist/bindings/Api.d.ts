import type { ApiResult, FetchFn, UserId, UserView, UserProfileView, ExternalUserProfile, TicketId, CreateTicket, SendTicketMessage, TicketView, TicketListingViewExpandedItem, TicketStatus, TelegramLoginData, CreateGroup, GroupView, GroupId } from "../";
export declare function generateId(): string;
export declare class Api {
    #private;
    private fetch;
    constructor(fetch: FetchFn);
    internalCreateUser(id: UserId, profile: ExternalUserProfile): Promise<ApiResult<null>>;
    internalFakeLogin(userId: UserId): Promise<ApiResult<null>>;
    telegramLogin(data: TelegramLoginData): Promise<ApiResult<null>>;
    getMe(): Promise<ApiResult<UserView>>;
    getUserProfile(id: UserId): Promise<ApiResult<UserProfileView>>;
    getUserGroups(id: UserId): Promise<ApiResult<GroupView[]>>;
    createGroup(id: GroupId, creation: CreateGroup): Promise<ApiResult<null>>;
    getGroupTickets(id: GroupId): Promise<ApiResult<Array<TicketListingViewExpandedItem>>>;
    getGroup(id: GroupId): Promise<ApiResult<GroupView>>;
    addGroupMember(id: GroupId, new_member: UserId): Promise<ApiResult<null>>;
    createTicket(id: TicketId, creation: CreateTicket): Promise<ApiResult<null>>;
    getTicket(id: TicketId): Promise<ApiResult<TicketView>>;
    getOwnedTickets(): Promise<ApiResult<Array<TicketListingViewExpandedItem>>>;
    getAssignedTickets(): Promise<ApiResult<Array<TicketListingViewExpandedItem>>>;
    sendTicketMessage(id: TicketId, message: SendTicketMessage): Promise<ApiResult<null>>;
    changeTicketStatus(id: TicketId, new_status: TicketStatus): Promise<ApiResult<null>>;
    changeTicketAssignee(id: TicketId, new_assignee: UserId | null): Promise<ApiResult<null>>;
}
export default Api;

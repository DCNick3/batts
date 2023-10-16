import type { ApiResult, FetchFn, UserId, UserView, UserProfileView, ExternalUserProfile, TicketId, CreateTicket, SendTicketMessage, TicketView, TicketListingViewExpandedItem, TicketStatus } from "../";
export declare function generateId(): string;
export declare class Api {
    #private;
    private fetch;
    constructor(fetch: FetchFn);
    internalCreateUser(id: UserId, profile: ExternalUserProfile): Promise<ApiResult<null>>;
    internalFakeLogin(userId: UserId): Promise<ApiResult<null>>;
    getMe(): Promise<ApiResult<UserView>>;
    getUserProfile(id: UserId): Promise<ApiResult<UserProfileView>>;
    createTicket(id: TicketId, creation: CreateTicket): Promise<ApiResult<null>>;
    getTicket(id: TicketId): Promise<ApiResult<TicketView>>;
    getOwnedTickets(): Promise<ApiResult<Array<TicketListingViewExpandedItem>>>;
    getAssignedTickets(): Promise<ApiResult<Array<TicketListingViewExpandedItem>>>;
    sendTicketMessage(id: TicketId, message: SendTicketMessage): Promise<ApiResult<null>>;
    changeTicketStatus(id: TicketId, new_status: TicketStatus): Promise<ApiResult<null>>;
}
export default Api;

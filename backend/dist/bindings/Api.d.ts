import type { ApiResult, FetchFn, UserId, UserView, UserProfileView, ExternalUserProfile, TicketId, CreateTicket, TicketView } from '../index.ts';
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
}
export default Api;

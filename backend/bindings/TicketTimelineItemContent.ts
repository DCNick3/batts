// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { TicketStatus } from "./TicketStatus";
import type { UserId } from "./UserId";

export type TicketTimelineItemContent = { "type": "Message", from: UserId, text: string, } | { "type": "StatusChange", old: TicketStatus, new: TicketStatus, } | { "type": "AssigneeChange", old: UserId | null, new: UserId | null, };
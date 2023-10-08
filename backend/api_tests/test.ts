import { test } from 'uvu';
import * as assert from 'uvu/assert';
import nodeFetch from 'node-fetch';
import makeFetchCookie from 'fetch-cookie';

import Api, { generateId } from '../bindings/Api';
import type FetchFn from "../bindings/FetchFn";
import type {ApiError, ApiResult} from "../bindings/ApiResult";
import type {UserId} from "../bindings/UserId";

const BASE_URL = "http://localhost:3000";

function makeApi(): Api {
    const cookiedFetch = makeFetchCookie(nodeFetch);
    const testFetch: FetchFn = async (url, init) => {
        const resp = await cookiedFetch(BASE_URL + url, init);
        // console.log(url, resp.headers);
        return resp;
    };
    return new Api(testFetch);
}

function unwrap<T>(result: ApiResult<T>): T {
    if (result.status == 'Success') {
        return result.payload
    } else {
        throw new Error("Api returned an error:\n" + result.payload.report)
    }
}

function unwrapErr<T>(result: ApiResult<T>): ApiError {
    if (result.status == 'Success') {
        throw new Error("Api returned a success, while expected an error")
    } else {
        return result.payload
    }
}

async function makeFakeUser(api: Api): Promise<UserId> {
    const userId = generateId();
    unwrap(await api.internalCreateUser(userId, {
        type: "Telegram",
        id: 123456,
        first_name: "Edward",
        last_name: "Snowden",
        username: null,
        photo_url: null,
    }));

    unwrap(await api.internalFakeLogin(userId));

    return userId;
}

test('get_me', async () => {
    const api = makeApi();
    const userId = await makeFakeUser(api);

    const me = unwrap(await api.getMe())

    assert.is(me.id, userId);
    assert.is(me.identities.university, null);
    assert.is(me.identities.telegram.id, 123456);
    assert.is(me.identities.telegram.first_name, "Edward");
    assert.is(me.identities.telegram.last_name, "Snowden");
    assert.is(me.identities.telegram.username, null);
    assert.is(me.identities.telegram.photo_url, null);
});

test('create_ticket', async () => {
    const api = makeApi();
    const userId = await makeFakeUser(api);
    const ticketId = generateId();

    unwrap(await api.createTicket(ticketId, {
        title: "Everything is broken",
        body: "I can't do anything",
    }));

    const ticket = unwrap(await api.getTicket(ticketId));
    assert.is(ticket.id, ticketId);
    assert.is(ticket.title, "Everything is broken");
    assert.is(ticket.timeline.length, 1);
    const timelineItem = ticket.timeline[0];
    assert.is(timelineItem.content.type, "Message");
    if (timelineItem.content.type === "Message") {
        assert.is(timelineItem.content.text, "I can't do anything");
        assert.is(timelineItem.content.from, userId);
    }
})

test.run();
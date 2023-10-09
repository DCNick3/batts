import { expect, test } from 'vitest'

import nodeFetch from 'node-fetch';
import makeFetchCookie from 'fetch-cookie';

import { Api, generateId, FetchFn, UserId, ApiError, ApiResult } from "@";

const BASE_URL = "http://localhost:3000";

function makeApi(): Api {
    const loggingFetch: FetchFn = async (url, init) => {
        const requestType = init?.method ?? "GET";
        let headers = init?.headers ?? [];
        let headersArr: [string, string][];
        if (typeof headers === 'object') {
            headersArr = Object.entries(headers);
        } else {
            headersArr = headers;
        }
        const headersStr = (headersArr.length > 0)
            ? headersArr.map(([k, v]) => `${k}: ${v}`).join("\n") + "\n"
            : '';
        const bodyStr = (init?.body !== undefined)
            ? init.body + '\n\n'
            : '';

        const rq = `\n###\n${requestType} ${url}\n${headersStr}\n${bodyStr}`;

        const response = await nodeFetch(url, init);

        const statusLine = `HTTP/1.1 ${response.status} ${response.statusText}`;
        const responseHeaders = Object.entries(response.headers.raw());
        const headersStr2 = responseHeaders.map(([k, v]) => `${k}: ${v}`).join("\n");

        const resp = `${statusLine}\n${headersStr2}\n\n${await response.clone().text()}`;

        console.debug(rq + resp);

        return response;
    };

    const cookiedFetch = makeFetchCookie(loggingFetch);
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
    console.log("Creating fake user", userId);
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

test("get_me", async () => {
    const api = makeApi();
    const userId = await makeFakeUser(api);

    const me = unwrap(await api.getMe())

    expect(me.id).toBe(userId);
    expect(me.identities.university).toBe(null);
    expect(me.identities.telegram.id).toBe(123456);
    expect(me.identities.telegram.first_name, "Edward");
    expect(me.identities.telegram.last_name, "Snowden");
    expect(me.identities.telegram.username).toBe(null);
    expect(me.identities.telegram.photo_url).toBe(null);
})
test("create_ticket", async () => {
    const api = makeApi();
    const userId = await makeFakeUser(api);
    const ticketId = generateId();

    unwrap(await api.createTicket(ticketId, {
        title: "Everything is broken",
        body: "I can't do anything",
    }));

    const ticket = unwrap(await api.getTicket(ticketId));
    expect(ticket.id).toBe(ticketId);
    expect(ticket.title).toBe("Everything is broken");
    expect(ticket.timeline.length).toBe(1);
    const timelineItem = ticket.timeline[0];
    expect(timelineItem.content.type, "Message");
    if (timelineItem.content.type === "Message") {
        expect(timelineItem.content.text).toBe("I can't do anything");
        expect(timelineItem.content.from).toBe(userId);
    }
})

test("make_mock_tickets", async () => {
    const api1 = makeApi();
    const api2 = makeApi();

    const userId1 = "MekGz7Af4HwV8uwBm7c82P";
    const userId2 = "R9wBXTwKPgNXGxVzcvo8xv";
    const ticket1 = "F2VaZtXgAKgxJncCbMbX9V";
    const ticket2 = "JrBVVdGiKLHEeaPApdq236";
    const ticket3 = "6cbfgbg2E3FGNLZNxxt7Nv";
    const ticket4 = "JrBVVdGiKLHEeaPApdq236";

    await api1.internalCreateUser(userId1, {
        type: "Telegram", id: 123456,
        first_name: "Edward", last_name: "Snowden",
        username: null, photo_url: null,
    });
    await api2.internalCreateUser(userId2, {
        type: "Telegram", id: 123456,
        first_name: "Edward", last_name: "Snowden",
        username: null, photo_url: null,
    });

    await api1.internalFakeLogin(userId1);
    await api2.internalFakeLogin(userId2);

    if ((await api1.createTicket(ticket1, {
        title: "Broken chair",
        body: "Hello,\n\nI'm writing to you because the chair in the room 123 is broken. Please fix it.",
    })).status == "Success") {
        await api2.sendTicketMessage(ticket1, {
            body: "Hey, we'll fix it soon."
        })
    }

    await api1.createTicket(ticket2, {
        title: "No internet",
        body: "Hello,\n\nI'm writing to you because there is no internet in the room 123. Please fix it.",
    });
    await api2.createTicket(ticket3, {
        title: "Broken bulb",
        body: "Hello,\n\nI'm writing to you because the light bulb in the room 123 is broken. Please fix it.",
    });
    await api2.createTicket(ticket4, {
        title: "Dashboard broken",
        body: "Hello,\n\nI'm writing to you because the dashboard is broken. Please fix it.",
    });
})
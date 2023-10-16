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
        destination: "ItDepartment",
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

    const myTickets = unwrap(await api.getOwnedTickets());
    expect(myTickets.length).toBe(1);
    expect(myTickets[0].id).toBe(ticketId);
    expect(myTickets[0].title).toBe("Everything is broken");
    expect(myTickets[0].status).toBe("Pending");
    expect(myTickets[0].destination).toBe("ItDepartment");
    expect(myTickets[0].owner).toBe(userId);
    expect(myTickets[0].assignee).toBe(null);

    const assignedTickets = unwrap(await api.getAssignedTickets());
    expect(assignedTickets.length).toBe(0);
})

test("telegram_login", async () => {
    const api = makeApi();
    const result = await api.telegramLogin({
        "id": 519776851,
        "first_name": "Anatoliy",
        "last_name": "Baskakov",
        "username": "Nihon_V",
        "photo_url": "https:\/\/t.me\/i\/userpic\/320\/MALv0jB4u8C_pdnrnxBygB9WiLfr7kfkas1yOfr4jQg.jpg",
        "auth_date": 1697473082,
        "hash": "922e211defd4965842f14a93c93c36ec03c6310cc6cc633f8036390e94e6935a"
    });
    // we expect an error because the auth is too old
    expect(result.status).toBe("Error");
});

test("make_mock_tickets", async () => {
    const api1 = makeApi();
    const api2 = makeApi();

    const userId1 = "MekGz7Af4HwV8uwBm7c82P";
    const userId2 = "R9wBXTwKPgNXGxVzcvo8xv";
    const ticket1 = "F2VaZtXgAKgxJncCbMbX9V";
    const ticket2 = "H3NbS5NeKY33AMr6Pvtw6H";
    const ticket3 = "XYF1Ur6Z4oeVBioYtW62nF";
    const ticket4 = "M1A5QazKGRUNoTqraWxYou";
    const ticket5 = "BJytpHn3GssUW24WJJkrPg";

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
        destination: "DormManager",
        title: "Broken chair",
        body: "Hello,\n\nI'm writing to you because the chair in the room 123 is broken. Please fix it.",
    })).status == "Success") {
        await api2.sendTicketMessage(ticket1, {
            body: "Hey, we'll fix it soon."
        })
        await api1.sendTicketMessage(ticket1, {
            body: "Thanks, I'll be waiting."
        })
    }

    await api1.createTicket(ticket2, {
        destination: "ItDepartment",
        title: "No internet",
        body: "Hello,\n\nI'm writing to you because there is no internet in the room 123. Please fix it.",
    });
    await api1.createTicket(ticket3, {
        destination: "DormManager",
        title: "Doorknob",
        body: "Hello,\n\nI'm writing to you because the doorknob in the room 123 is broken. Please fix it.",
    });
    await api1.createTicket(ticket4, {
        destination: "ItDepartment",
        title: "Broken bulb",
        body: "Hello,\n\nI'm writing to you because the light bulb in the room 123 is broken. Please fix it.",
    });
    await api1.createTicket(ticket5, {
        destination: "ItDepartment",
        title: "Dashboard broken",
        body: "Hello,\n\nI'm writing to you because the dashboard is broken. Please fix it.",
    });
})
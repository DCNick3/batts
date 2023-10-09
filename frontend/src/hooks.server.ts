import type { Handle } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";

const BACKEND_URL = env.BACKEND_URL ?? (() => { throw new Error("BACKEND_URL not set") })();
const API_PROXY_PATH = "/api";

const handleApiProxy: Handle = async ({ event }) => {
  // const origin = event.request.headers.get("Origin");

  // // reject requests that don't come from the webapp, to avoid your proxy being abused.
  // if (!origin || new URL(origin).origin !== event.url.origin) {
  //   throw new Error("Origin not allowed");
  //   // throw error(403, "Request Forbidden.");
  // }

  // build the new URL path with your API base URL, the stripped path and the query string
  const urlPath = `${BACKEND_URL}${event.url.pathname}${event.url.search}`;
  const proxiedUrl = new URL(urlPath);

  // Strip off header added by SvelteKit yet forbidden by underlying HTTP request
  // library `undici`.
  // https://github.com/nodejs/undici/issues/1470
  event.request.headers.delete("connection");

  return fetch(proxiedUrl.toString(), {
    // propagate the request method and body
    // @ts-expect-error some required undici updates...
    duplex: event.request.duplex,
    body: event.request.body,
    method: event.request.method,
    headers: event.request.headers,
  }).catch((err) => {
    console.log("Could not proxy API request: ", err);
    throw err;
  });
};

export const handle: Handle = async ({ event, resolve }) => {
  // intercept requests to `/api-proxy` and handle them with `handleApiProxy`
  if (event.url.pathname.startsWith(API_PROXY_PATH)) {
    return handleApiProxy({event, resolve});
  }

  return await resolve(event);
};

<img src="images/favicon.png" width="100" height="100">

# BATTS
### Best Automated Ticket Tracking System

Never get lost in the ticketing jungle!

## Description

BATTS is a ticketing system in development by students of Innopolis University

Its end goal is to unite all of the various ticketing systems that exist in Innopolis by developing a new, universal one written using more maintainable stack that php.

## Features

Right now BATTS supports the very basics of ticketing: you can create groups and send tickets to them for processing.

Autorization is performed though Telegram.

BATTS is designed to be intuitive and simple to use. If you do not understand how to do X, open an issue and we will figure it out.

## Demo

You can visit our deployed instance here: https://batts.tatar/

Be careful however, it updates frequently and we wipe it quite often

![BATTS Home Screen](images/homescreen.png?raw=true "BATTS Home Screen")
![BATTS Ticket View](images/ticket.png?raw=true "BATTS Ticket View")
![BATTS Groups UI](images/groups.png?raw=true "BATTS Groups UI")


## Technological Stack

The project, as of now, consists of the following components:

* True Backend -- written in Rust using [Actix](https://actix.rs/), following [CQRS framework](https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs) for data storage
* Front-Back -- [Sveltekit](https://kit.svelte.dev/), used for Server Side Rendering and routing
* True Frontend -- [Svelte](https://svelte.dev/) + Typescript + [Tailwind](https://tailwindcss.com/), using some of the [Flowbite](https://flowbite.com/) components
* Analytics Engine [PostHog](https://posthog.com/)
* Search Engine [Meilisearch](https://www.meilisearch.com/)
* Object store [MinIO](https://min.io/docs/minio/linux/index.html)

We put a lot of emphasis on keeping our technological stack modern and as usable as possible.


## How to deploy

We provide dockerfiles and docker images for the backend and frontend. We also provide the Kubernetes manifests we use.

The only thing that we do not provide for you is PostHog deployment as it is best [to follow their instructions](https://posthog.com/docs/self-host) as their product is massive.

### [*For customer*] How to deploy for development purposes

If what you want is to deploy the app to develop it here is what you can do.

#### [*For customer*] Developing frontend

To try developing frontend you need to first set up the backend. For it you enter `backend` folder and use `docker-compose up -d`, it should build and make backend active.

Then you need to set up authorization. Since usual authorization does not work in dev environment set up a mock one.

First create a user:

```
curl -X PUT --location "http://localhost:3000/api/users/FDUeanyKADQEpyrydYn7XB" \
    -H "Content-Type: application/json" \
    -d "{
          \"profile\": {
            \"type\": \"Telegram\",
            \"id\": 123456,
            \"first_name\": \"Edward\",
            \"last_name\": \"Snowden\"
          }
        }"
```

Second get a cookie for the created user:

```
curl -vvv -X POST --location "http://localhost:3000/api/fake-login/FDUeanyKADQEpyrydYn7XB"
```

Third step is to add this cookie to your browser. You would want a browser extension or [addon](https://addons.mozilla.org/en-US/firefox/addon/cookie-quick-manager/) for that.

In frontend directory create (or edit) `.env` file to contain:

```
BACKEND_URL=http://localhost:3000
```

After this you are good to use `yarn` and `yarn dev` and begin working on the frontend.

#### [*For customer*] Developing backend
In case you want to develop backend, the process is as follows:

First set up meilisearch:

```
docker run -it --rm \
           -p 7700:7700 \
           -e MEILI_ENV='development' \
           getmeili/meilisearch:v1.4 -d --name meilisearch
```

*Near future* you may need to set up minio for object storage:
```
docker run -dt                                  \
  -p 9000:9000 -p 9090:9090                     \
  --name "minio_local"                          \
  minio server --console-address ":9090"
```

After this export the environment variable:

```
ENVIRONMENT=dev
```

Then you are good to use `cargo run` to build and run the backend.


## [*For customer*] How to contribute

For now the project is in active development, but after that you may contribute to it by either forking it, or submitting issues and merge requests to cover them. Please make sure that your code at least passess relevant linters if you want it to be accepted and make sure to test your version locally for both desktop and mobile.

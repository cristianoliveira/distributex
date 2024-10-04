# distributex

This is a POC of a federated architecture combining the Xs of NGINX and HTMX 

(AKA microfrontends/microservices same thing)

## Architecture

![Architecture](/architecture.png?raw=true "architecture diagram")

## Tech stack

 - Main app: HTMX, NGINX, and Tailwind
 - Service 1 (Todo list) - Golang & Sqlite
 - Service 2 (Chat) - Rust & Websockets
 - Service 3 (Tech News) - Python & Hackernews API
 - Docker 
 
## Why?

I wanted to understand how much I could stretch the idea and how hard would be to have micro frontends and services using 
HTMX. Spoiler alert, it's surprisingly easy due to [HATEOAS](https://htmx.org/essays/hateoas/).

## Main goals

As a dev, you should be able to do/have:

 - A DX & UX isolation, which means accessing the service URL gives you the same UX as in the main APP.
 - Freedom of choice, it shouldn't matter the stack you pick for the services as long you respect the protocol
 - Communication via events not only on the backend but also on the frontend side, thanks to an HTTP header convention ([HX-trigger](https://htmx.org/headers/hx-trigger/))

## Current state

 - [x] Main app
   - [x] Serve index.html 
   - [x] Proxy requests to services
   - [x] Different clients/themes
      - http://localhost:4040/index.html
      - http://localhost:4040/retro.html   
 - [x] Todo service
   - [x] Add item
   - [x] Remove item
   - [x] Mark as done
   - [ ] Make it reactive to changes from other sessions (websocket?)
 - [x] Chat service 
   - [x] Allow pick nickname
   - [x] Join a room
   - [x] Send and receive messages
 - [x] News service
   - [x] Allow fetch latest news
   - [x] Allow managing favorite news
 - [x] Main app serving widgets
   - [x] Todo service
   - [x] Chat service
   - [x] News service
   - [ ] Fallback for service loading
   - [ ] Handle errors
 - [x] Implement the 3rd service 
 - [x] Cross services communication via events
   - Events emmited by one app is able to trigger another app to do something
     Eg: When a user adds a new todo the chat app sends a message to the room.
     See [HX-trigger](https://htmx.org/headers/hx-trigger/)
 - [ ] Describe infra and deploy a demo to AWS

## Getting started

### Requirements

Make sure you have 
 - [docker](https://docs.docker.com/engine/install/)
 - [docker compose](https://docs.docker.com/compose/install/)

If you are on OSX we recommend installing [colima](https://github.com/abiosoft/colima) as a container runtime

### Running

Check the available commands with:
```bash
make help
```

To start all the application run:

```bash
make start
```

That's it, you should be able to access the main application at http://localhost:4040

You also should be able to access the microservices at:
  - Todo list manager - http://localhost:4001/
  - Chat service - http://localhost:4002/
  - Tech News reader - http://localhost:4003/

### Live reload mode

You need [funzzy](https://github.com/cristianoliveira/funzzy) (sorry?)

First, start the services with `make start` then in another terminal do:

```bash
make watch
```

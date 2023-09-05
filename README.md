# distributed-htmx
This is a POC of a federated application using HTMX and NGINX

## Architecture

![Architecture](/architecture.png?raw=true "Optional Title")

## Tech stack

 - HTMX
 - Tailwind
 - NGINX
 - Docker 
 - Service 1 (Todo list) - Golang & Sqlite
 - Service 2 (Chat) - Rust & Websockets
 - Service 3 (??) - Nextjs??

## Current state

 - [x] Main app serving index.html 
 - [x] Main app proxying requests to services
 - [x] Todo service
   - [x] Add item
   - [x] Remove item
   - [x] Mark as done
   - [ ] Make it reactive to changes from other sessions (websocket?)
 - [x] Chat service 
   - [x] Allow pick nickname
   - [x] Join a room
   - [x] Send and receive messages
 - [x] Main app serving widgets
   - [x] Todo service
   - [x] Chat service
   - [ ] 3rd service
 - [ ] Implement the 3rd service 

## Getting started

### Requirements

Make sure you have 
 - [docker](https://docs.docker.com/engine/install/)
 - [docker compose](https://docs.docker.com/compose/install/)

If you are on OSX we recommend installing [colima](https://github.com/abiosoft/colima) as a container runtime

### Running

```bash
make start
```

That's it, you should be able to access the main application at http://localhost:4040

You also should be able to access the micro services at:
  - Todo list manager - http://localhost:4001/todo
  - Chat service - http://localhost:4002/chat
  - Service 3 TBD - http://localhost:4003

### Live reload mode

You need [funzzy](https://github.com/cristianoliveira/funzzy) (sorry?)

First start the services with `make start` then in another terminal do:

```bash
make watch
```

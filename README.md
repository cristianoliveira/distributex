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

 - [x] Main app working and proxying requests to services
 - [x] Todo service - allow Add, Mark done, and delete items
 - [x] Chat service - allow pick nickname, join a room, and chat
 - [x] The integration of the services in the main app
 - [ ] Make Todo service more reactive using htmx-events
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

You need [funzzy](https://github.com/cristianoliveira/funzzy)

First start the services with `make start` then in another terminal do:

```bash
make watch
```

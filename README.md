# distributed-htmx
This is a POC of a federated application using HTMX and NGINX

## Architecture

![Architecture](/architecture.png?raw=true "Optional Title")

## Tech stack

 - HTMX
 - Tailwind
 - NGINX
 - Golang
 - Sqlite

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
  - todo list manager - http://localhost:4001
  - TBD 1 - http://localhost:4002
  - TBD 2 - http://localhost:4003

### Live reload mode

You need [entr](https://github.com/eradman/entr)

First start the services with `make start` then in another terminal do:

```bash
make <arg>
```

Where `arg` is one of 

 - watch-todos-app

# distributed-htmx
This is a POC of a federated application using HTMX and NGINX

## Architecture


<img src="https://raw.githubusercontent.com/cristianoliveira/distributed-htmx/main/architecture.png?token=GHSAT0AAAAAACG6JEPYHSYGB6VACS3W4AC2ZHRXRRQ" align="center" />

## Tech stack

 - HTMX
 - Tailwind
 - NGINX
 - Golang
 - Sqlite

## Getting started

### Requirements

Make sure you have 
 - [docker](https://docs.docker.com/engine/install/)https://docs.docker.com/engine/install/
 - [docker compose](https://docs.docker.com/compose/install/)https://docs.docker.com/compose/install/

### Running

```bash
docker compose up --build
```

That's it, you should be able to access the main application at http://localhost:4040

You also should be able to access the micro services at:
  - todo list manager - http://localhost:4001
  - TBD 1 - http://localhost:4002
  - TBD 2 - http://localhost:4003

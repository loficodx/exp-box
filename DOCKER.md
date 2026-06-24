# Docker setup for exp-box

This project runs these services:

- `frontend` — Vite/React app served by nginx on `http://localhost`
- `gateway` — Rust/Axum API on `http://localhost:8000`
- `auth-service` — internal Rust/Axum auth API
- `room-rce` — internal vulnerable RCE room service
- `room-xss` — internal placeholder XSS room service

## Start

```bash
cp .env.example .env

docker compose up --build
```

Open:

```text
http://localhost
```

The frontend calls the gateway through nginx:

```text
/api/health -> gateway:8000/api/health
```

The browser should call only `frontend` and `/api/...` paths. Room services are
reachable only inside Docker on `targets_net`.

## Useful commands

```bash
# Stop containers, keep database volume
docker compose down

# Stop containers and delete database volume
docker compose down -v

# Rebuild from scratch
docker compose build --no-cache

docker compose up
```

Build or start individual services:

```bash
docker compose build gateway
docker compose build room-rce
docker compose build room-xss

docker compose up gateway auth-service room-rce
docker compose up room-xss
```

Quick validation:

```bash
curl http://localhost:8000/api/health
curl http://localhost/api/health

curl -X POST http://localhost:8000/api/rooms/rce/actions/exec \
  -H 'Content-Type: application/json' \
  -d '{"cmd":"id"}'
```

## Database connection inside Docker

The gateway receives this environment variable:

```text
DATABASE_URL=sqlite://data/exp-box.db
```

The auth service uses its own SQLite database volume.

## Repository layout

```text
services/
  gateway/        # public API, auth proxy, room registry, progress
  auth/           # users, password hashing, sessions
  rooms/
    rce/          # vulnerable RCE room service
    xss/          # placeholder XSS room service
frontend/
docker-compose.yml
```

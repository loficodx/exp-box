# Docker setup for exp-box

This project runs four services:

- `frontend` — Vite/React app served by nginx on `http://localhost`
- `gateway` — Rust/Axum API on `http://localhost:8000`
- `auth-service` — internal Rust/Axum auth API
- `room-rce` — internal vulnerable RCE room service

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

## Database connection inside Docker

The gateway receives this environment variable:

```text
DATABASE_URL=sqlite://data/exp-box.db
```

The auth service uses its own SQLite database volume.

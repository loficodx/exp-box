# exp-box

`exp-box` is a local pentesting training platform for practicing web security concepts in isolated vulnerable rooms.

The project uses a service-oriented layout:

- `frontend` - React/Vite UI served by nginx.
- `gateway` - public Rust/Axum API, auth proxy, room registry, and progress tracking.
- `auth-service` - internal Rust/Axum service for users, password hashing, and cookie sessions.
- `room-rce` - internal vulnerable RCE training room.
- `room-xss` - internal XSS/CSRF training room placeholder, intended for stored XSS and CSRF exercises.

Only the frontend and gateway are exposed to the host. Auth and room services stay internal, and browser traffic should go through `/api/...` routes.

## Architecture

```text
browser
  |
  v
frontend / nginx
  |
  v
gateway
  |-----------------> auth-service
  |-----------------> room-rce
  |-----------------> room-xss
  |
  v
gateway database
```

## Run Locally

```bash
cp .env.example .env
docker compose up --build
```

Open:

```text
http://localhost
```

Useful checks:

```bash
curl http://localhost:8000/api/health
curl http://localhost/api/health
```

More Docker commands are documented in `DOCKER.md`.

## Repository Layout

```text
services/
  gateway/        # public API, auth proxy, room registry, progress
  auth/           # users, password hashing, sessions
  rooms/
    rce/          # vulnerable RCE room service
    xss/          # XSS/CSRF room placeholder
frontend/
docker-compose.yml
```

## Notes

This project intentionally contains vulnerable room services for local training. Keep vulnerable behavior scoped to room services and room-specific frontend pages. Do not expose internal room services directly to the host.

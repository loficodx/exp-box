# Docker setup for exp-box

This project runs three services:

- `frontend` — Vite/React app served by nginx on `http://localhost`
- `backend` — Rust/Axum API on `http://localhost:8000`
- `postgres` — PostgreSQL database on `localhost:5432`

## Start

```bash
cp .env.example .env

docker compose up --build
```

Open:

```text
http://localhost
```

The frontend calls the backend through nginx:

```text
/api/health -> backend:8000/api/health
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

The backend receives this environment variable:

```text
DATABASE_URL=postgres://exp_box:exp_box_password@postgres:5432/exp_box
```

At the moment the backend does not use PostgreSQL yet, but the database service is ready for the next step.

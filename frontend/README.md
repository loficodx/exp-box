# exp-box frontend

React/Vite frontend for the local exp-box training platform.

In Docker, nginx serves the built frontend on `http://localhost` and proxies
all `/api/...` requests to the internal `gateway` service:

```text
/api/* -> gateway:8000/api/*
```

The browser should use relative `/api/...` URLs. It should not call
`auth-service`, `room-rce`, or `room-xss` directly.

## Local commands

```bash
npm --prefix frontend install
npm --prefix frontend run build
npm --prefix frontend run lint
```

For the full platform, use Docker Compose from the repository root:

```bash
docker compose up --build
```

# DevOps and Deployment

## 1. Docker services

Recommended Docker Compose services:

```text
api
web
postgres
redis
worker
openclaw
reverse-proxy
```

## 2. docker-compose.template.yml

See `docker-compose.template.yml` in this package.

## 3. Environment variables

See `.env.example`.

## 4. Production recommendations

- Use managed PostgreSQL or dedicated PostgreSQL volume.
- Use HTTPS via Nginx/Caddy.
- Use separate database user for application.
- Do not expose PostgreSQL publicly.
- Store secrets in environment secret manager where possible.
- Mount storage folder for invoice attachments.
- Enable structured logging.

## 5. Logging

Backend should log:

- request id;
- user id;
- company id;
- endpoint;
- status code;
- duration;
- error details without leaking secrets.

Recommended crates:

- `tracing` + `tracing-subscriber`
- `tracing-loki` / Grafana Loki
- Structured JSON logs (tower-http trace layer)

## 6. Background jobs

Use worker service for:

- report schedules;
- tax due reminders;
- overdue invoice alerts;
- document OCR processing;
- Excel import jobs;
- backup verification task.

## 7. Migration process

Use SQLx migrations (plain `.sql` files in `migrations/` directory).

Process:

```text
local migration -> test -> staging -> production
```

Run migrations with `sqlx migrate run`. Do not auto-run destructive migrations in production without review.

## 8. Deployment checklist

- [ ] Environment variables configured.
- [ ] Database migration applied.
- [ ] Admin user created.
- [ ] OpenClaw channel configured.
- [ ] Webhook/service token configured.
- [ ] Storage path writable.
- [ ] Backup schedule enabled.
- [ ] HTTPS enabled.
- [ ] Health checks pass.
- [ ] First company setup completed.

## 9. Health checks

Backend endpoints:

```text
GET /health
GET /health/db
GET /health/storage
```

## 10. Monitoring alerts

Alert on:

- API down;
- PostgreSQL down;
- failed background job;
- backup failed;
- disk usage > 80%;
- repeated AI tool errors;
- failed login spike.

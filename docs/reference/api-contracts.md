# API Contracts (Current Baseline)

## Routes
- `GET /health`
- `POST /api/v1/chat`
- `GET /api/v1/config`
- `GET /api/v1/plugins`
- `GET /api/v1/logs`

## Auth policy
- Bearer token only.
- Query-token auth is disallowed.
- Empty token is rejected.

## CORS policy
- Deny by default when no allowlist is configured.
- Wildcard origin is rejected in configuration validation.

## Adapter diagnostics fields
- `platform`
- `queue_depth`
- `dropped_messages`
- `import_records`

## Import contract
Each platform adapter exposes:
1. `discover_sources()`
2. `parse_authorized_export(raw)`
3. `normalize_messages(records, platform)`
4. `checkpoint()`
5. `diagnostics_note()`

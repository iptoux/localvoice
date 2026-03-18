# ADR-003: SQLite for Local Data Persistence

## Status
Accepted

## Context
The application requires local storage for:
- Session history (transcripts, metadata)
- Dictionary entries and correction rules
- Ambiguous terms tracking
- Model installations registry
- User settings

Options considered:
- **SQLite**: Single-file relational database
- **JSON files**: Flat file storage
- **Embedded KV store**: RocksDB, sled
- **Full database server**: PostgreSQL (rejected: requires external setup)

## Decision
Use **SQLite** as the single local database for all persistence needs.

## Consequences

### Positive
- **Single file**: All data in one `localvoice.db` file, easy to backup/export
- **No setup**: Embedded, no server installation required
- **Portable**: Works across Windows, macOS, Linux
- **Mature**: Excellent Rust support via `rusqlite` or `sqlx`
- **Relational**: Proper schema with foreign keys, indexes, queries
- **ACID**: Transactions ensure data integrity
- **Queryable**: Complex queries for history search, stats aggregation

### Negative
- **Single file limitations**: Not ideal for huge datasets (acceptable for personal use)
- **Concurrency**: Write locking (mitigated by single-user app)
- **No network**: Cannot share data between machines (intentional: offline-first)

### Trade-offs
- Accept single-file limitations in exchange for zero-configuration deployment
- Use proper schema design to handle scale up to thousands of sessions
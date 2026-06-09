# PlaylistHistory

A Rust service that archives how editorial playlists on music platforms change over time.

Streaming platforms only expose the current state of a playlist. PlaylistHistory periodically captures that state and records the differences: which tracks were added, which were removed, and when. The historized data is then meant to be served through an API.

This is also a learning project. I work mainly with Node.js and NestJS, and I am using this repository to learn Rust by building something real rather than following tutorials. The goal is to apply the same engineering practices I use professionally (clear boundaries, strong typing, enforced linting) while learning the language and its ecosystem.

## Status

Early prototype. The current build parses a captured Deezer playlist from a local JSON fixture and prints its tracks. There is no persistence, scheduling, or HTTP layer yet.

What exists today:

- `src/deezer.rs`: typed deserialization of Deezer's playlist JSON, using newtype wrappers around IDs and titles instead of raw primitives.
- `src/main.rs`: reads `DeezerPlaylist.json`, parses it into a typed model, and prints the result.

The empty modules (`db.rs`, `model.rs`, `sync.rs`, `error.rs`) mark the planned shape of the project and are not implemented yet.

## Roadmap

- [x] Parse Deezer playlist JSON into typed domain objects
- [ ] Fetch playlists live from the Deezer API
- [ ] Persist snapshots and compute diffs between them
- [ ] Schedule periodic syncs
- [ ] Expose the history through a web API
- [ ] Support additional platforms

## Design choices

- **Newtype pattern for identifiers.** IDs and names are wrapped in dedicated types (`PlaylistId`, `TrackId`, `PlaylistTitle`, `TrackTitle`) rather than passed around as `u64` or `String`, so values cannot be mixed up by accident.
- **Separation of concerns.** Platform specific parsing lives in its own module. Persistence, domain model, sync logic, and error types are kept in separate modules as the project grows.
- **Linting as a contract.** `clippy::pedantic` runs as warnings and `unsafe` code is forbidden at the crate level, so the constraints are enforced by the compiler rather than left to discipline.

## Tech stack

| Concern        | Choice                                  |
| -------------- | --------------------------------------- |
| Language       | Rust (edition 2024)                     |
| Async runtime  | Tokio                                   |
| HTTP client    | reqwest                                 |
| Database       | sqlx (target backend not yet finalized) |
| CLI            | clap                                    |
| Serialization  | serde, serde_json                       |
| Dates          | chrono                                  |
| Errors         | thiserror, anyhow                       |
| Logging        | tracing                                 |
| Config         | dotenvy                                 |

The database backend is still being decided: sqlx is currently configured for SQLite, while the Docker setup provisions PostgreSQL. This will be reconciled before persistence work begins.

## Running it

Requires a recent Rust toolchain (edition 2024).

```sh
cargo run        # parse DeezerPlaylist.json and print its tracks
cargo build
cargo test
cargo clippy     # pedantic lints are enabled
cargo fmt
```

A PostgreSQL container is defined in `docker-compose.yml` for future persistence work. It reads its settings from a `.env` file (`DB_API_*` variables) and is not required to run the current prototype.

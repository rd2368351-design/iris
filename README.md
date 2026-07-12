# Iris

Mail server built from scratch in Rust — SMTP, IMAP, and JMAP support, built module by module. Architecture inspired by [Stalwart](https://github.com/stalwartlabs/stalwart).

> **Status: early development.** The foundation layer (storage, config, auth) is being built first; protocol servers (SMTP/IMAP/JMAP) come after that's solid and tested.

## Why Iris

Most self-hosted mail servers are either decades-old and hard to extend (Postfix + Dovecot + a dozen glue scripts) or closed-source SaaS. Iris aims for a single, modern, modular codebase — one project you can actually read end to end.

## Architecture

Iris is a Cargo workspace. Each crate owns one responsibility, and crates only depend downward (never sideways or up):

```
crates/
├── types       shared data types (Id, EmailAddress, ...) — no dependencies
├── utils       small dependency-light helpers (codecs, time)
├── common      config loading, tracing/telemetry setup
├── store       pluggable storage (Store trait; SQLite backend first)
├── directory   accounts, authentication
└── main        binary entry point
```

More crates (`smtp`, `imap`, `jmap`, `spam-filter`, ...) get added as the project grows — see [Issues](../../issues) for the current roadmap.

## Building

```bash
cargo build
cargo test
```

No external services required yet — the default storage backend is a local SQLite file (`./data/iris.db`).

## License

Iris is licensed under the [GNU Affero General Public License v3.0](LICENSE). In short: you're free to use, modify, and redistribute Iris, but if you run a modified version as a network service, you must make your modified source available to its users too.

## Contributing

Not yet accepting external contributions — the core architecture is still settling. Feel free to open an issue for bugs or ideas.

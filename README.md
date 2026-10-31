# Cradex

Cradex is a local coding agent with Peril Rose branding. It is a fork of [OpenAI Codex](https://github.com/openai/codex).

## Getting started

Build and run Cradex from the Rust workspace:

```sh
cd codex-rs
cargo run --bin codex -- "explain this codebase to me"
```

The executable and some configuration names still use `codex` for compatibility. See [Installing & building](docs/install.md) for prerequisites and further build instructions.

Cradex can use supported model providers, including OpenAI. Authentication and model availability depend on the provider. The [upstream authentication documentation](https://developers.openai.com/codex/auth) describes the inherited OpenAI integration.

## Documentation

- [Getting started](docs/getting-started.md)
- [Configuration](docs/config.md)
- [Sandbox and approvals](docs/sandbox.md)
- [Contributing](docs/contributing.md)
- [Installing & building](docs/install.md)

Some documentation links point to the upstream Codex project and may describe behavior that differs from this fork.

This repository is licensed under the [Apache-2.0 License](LICENSE). See [NOTICE](NOTICE) for attribution.

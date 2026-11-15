# Porch

A work-in-progress authentication and identity system.

## Development Progress

Currently, only some basic functionality for user identity management and
passkey registration/authentication has been implemented.

The goal is to create a fully-fledged authentication system, allowing you to
offload auth from your apps to Porch. It can also be used as a common auth
provider for your self-hosted apps.

### Current Goals

- [ ] Login sessions
- [ ] Admin panel
- [ ] User registration flow
- [ ] Proxy auth

### Future Goals

- [ ] Authenticating via OIDC
- [ ] OIDC provider

## Build from source

### The frontend

The frontend is a SvelteKit app. To build it, you'll need [Node.js][nodejs]
and [pnpm][pnpm] installed.

```sh
cd frontend
pnpm install
pnpm build
```

The build outputs will appear under `frontend/build`.

### The server

The server is built using Rust. You'll need to install the Rust toolchain. The
easiest way to do that is using [Rustup][rustup].

To build the server, you must have already built the frontend, see the previous
section for how to do that.

```sh
cargo build --release
```

The compiled binary will appear under `target/debug`.

[nodejs]: https://nodejs.org/en
[pnpm]: https://pnpm.io/
[rustup]: https://rustup.rs/

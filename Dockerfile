# --- Build frontend ---
FROM node:24-alpine AS frontend

COPY --from=ghcr.io/pnpm/pnpm:12-alpine /opt/pnpm /opt/pnpm
RUN for bin in pnpm pn pnpx pnx; do \
    ln -s "/opt/pnpm/$bin" "/usr/local/bin/$bin"; \
  done

COPY frontend/package.json frontend/pnpm-lock.yaml frontend/pnpm-workspace.yaml .
RUN pnpm install --frozen-lockfile

COPY frontend .
RUN pnpm build

# --- Build server ---
FROM rust:1.100-alpine AS server

COPY Cargo.toml Cargo.lock .
COPY crates/porch/Cargo.toml crates/porch/
RUN mkdir crates/porch/src && touch crates/porch/src/lib.rs
RUN cargo build --release

COPY crates crates
RUN touch crates/porch/src/lib.rs
COPY --from=frontend build frontend/build
RUN cargo build --release

# --- Build final image ---
FROM scratch

COPY --from=server target/release/porch /bin/

ENTRYPOINT ["/bin/porch"]

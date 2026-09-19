# --- Stage 1 ---
FROM rust:1.96.0-slim-bookworm AS builder
WORKDIR /app

COPY crates/rustfinity-runner .
RUN cargo build --release

# --- Stage 2 ---

FROM rust:1.96.0-slim-bookworm AS runner
LABEL rustfinity-runner="true"

# Install OpenSSL development packages and pkg-config
RUN apt-get update && apt-get install -y \
    heaptrack \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY crates/syntest crates/syntest
COPY crates/rustfinity-runner/docker/Cargo.toml Cargo.toml

RUN cargo new challenges/playground && rm -rf challenges/playground/.git

WORKDIR /app/challenges/playground
RUN cargo add \
  syn \
  quote \
  tempfile@3.23.0
# Separate: with a pinned `name@version`, `cargo add` matches a qualified
# `--features tokio/full` against the literal spec rather than the crate name
# and fails with "feature `full` activated for crate `tokio` but the crate
# wasn't specified". One crate per invocation takes bare feature names.
RUN cargo add tokio@1.53.1 --features full,test-util

WORKDIR /app
RUN cargo build

COPY --from=builder /app/target/release/rustfinity-runner /app/

RUN chown -R 1000:1000 /app

USER 1000:1000

# The final structure:
# /app/rustfinity-runner (executable)
# /app/challenges/playground (project)
# /app/crates/syntest (library)

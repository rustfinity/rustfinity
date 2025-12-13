# --- Stage 1 ---
FROM rust:slim-bookworm AS builder
WORKDIR /app

COPY crates/rustfinity-runner .
RUN cargo build --release

# --- Stage 2 ---

FROM rust:slim-bookworm AS runner
LABEL rustfinity-runner="true"

# Install OpenSSL development packages and pkg-config
RUN apt-get update && apt-get install -y \
  heaptrack=1.5.0+dfsg1-5 \
  # in trixie these are needed to build some crates
  pkg-config=1.8.1-4 \
  libssl-dev=3.5.4-1~deb13u1 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY crates/syntest crates/syntest
COPY crates/rustfinity-runner/docker/Cargo.toml Cargo.toml

RUN cargo new challenges/playground && rm -rf challenges/playground/.git

WORKDIR /app/challenges/playground
RUN cargo add \
  syn \
  quote

WORKDIR /app
RUN cargo build

ENV CARGO_HOME=/home/myuser/.cargo
RUN mkdir -p $CARGO_HOME
COPY --from=builder /app/target/release/rustfinity-runner /app/

RUN chown -R 1000:1000 /app

USER 1000:1000

# The final structure:
# /app/rustfinity-runner (executable)
# /app/challenges/playground (project)
# /app/crates/syntest (library)

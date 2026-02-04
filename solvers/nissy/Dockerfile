FROM debian:trixie-slim AS nissy-builder
RUN apt-get update && apt-get install -y wget tar gzip make gcc
WORKDIR /opt/nissy
RUN wget https://nissy.tronto.net/nissy-2.0.8.tar.gz && \
    tar -xzf nissy-2.0.8.tar.gz --strip-components=1 && \
    make

FROM rust:1.90-slim-trixie AS rust-builder
WORKDIR /app
RUN --mount=type=bind,source=src,target=src \
    --mount=from=solvers-rs,target=/solvers-rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/nissy-* /bin/.

FROM debian:trixie-slim AS nissy-runtime-base
COPY --from=nissy-builder /opt/nissy/nissy /usr/local/bin/nissy


FROM nissy-runtime-base AS nissy-optimal
COPY --from=rust-builder /bin/nissy-optimal /bin/
ENTRYPOINT [ "/bin/nissy-optimal" ]

FROM nissy-runtime-base AS nissy-light
COPY --from=rust-builder /bin/nissy-light /bin/
ENTRYPOINT [ "/bin/nissy-light" ]

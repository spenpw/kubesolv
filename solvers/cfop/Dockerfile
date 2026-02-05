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
    cp ./target/release/cfop-exploration /bin/.

FROM debian:trixie-slim AS cfop-exploration
COPY --from=rust-builder /bin/cfop-exploration /bin/
ENTRYPOINT [ "/bin/cfop-exploration" ]

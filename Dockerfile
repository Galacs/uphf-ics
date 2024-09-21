FROM lukemathwalker/cargo-chef:latest-rust-1.81 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin uphf-ics

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/uphf-ics /usr/local/bin
ENTRYPOINT ["/usr/local/bin/uphf-ics"]

FROM rust:1.88.0-alpine3.22 AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev gcc

COPY . .

RUN rustup default nightly && cargo build --release

FROM alpine:3.22

WORKDIR /app

COPY --from=builder /app/target/release/ax-rabbitmq-exporter ax-rabbitmq-exporter

ENV RUST_LOG=info

ENTRYPOINT ["./ax-rabbitmq-exporter"]

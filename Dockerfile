FROM rust:1.87.0-alpine3.22 AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev gcc

COPY . .

RUN rustup default nightly && cargo build --release

FROM alpine:3.22

WORKDIR /app

COPY --from=builder /app/target/release/rabbitmq_exporter rabbitmq_exporter

ENV RUST_LOG=info

ENTRYPOINT ["./rabbitmq_exporter"]

FROM rust:1.89.0 AS builder

WORKDIR /app

COPY . .

RUN rustup default nightly && cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/ax-rabbitmq-exporter /

ENV RUST_LOG=info

ENTRYPOINT ["./ax-rabbitmq-exporter"]

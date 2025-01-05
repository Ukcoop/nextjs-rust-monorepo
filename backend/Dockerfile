FROM rust:1.83 AS builder

WORKDIR /app

COPY backend/ .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 3080

CMD ["./backend"]

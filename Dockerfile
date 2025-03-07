FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/pillable-api /app/pillable-api

EXPOSE 3000
CMD ["/app/pillable-api"]

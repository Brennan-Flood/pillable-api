FROM rust:1.75 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/pillable-api /app
COPY .env /app/.env

EXPOSE 3000
CMD ["./pillable-api"]

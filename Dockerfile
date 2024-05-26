FROM rust:1.78 AS Build
WORKDIR /app
COPY . .
RUN cargo test
RUN cargo build --release


FROM debian:trixie-slim AS Release
WORKDIR /app
COPY --from=Build /app/target/release/pong-transaction-service .
EXPOSE 8000
CMD ["./pong-transaction-service"]
ARG PORT=8084

FROM rust:1.78 AS Build
WORKDIR /app
COPY . .
RUN cargo test
RUN cargo build --release


FROM debian:trixie-slim AS Release
WORKDIR /app
COPY --from=Build /app/target/release/pong-transaction-service .
EXPOSE ${PORT}

ENV MONGODB_URI="ENTER SECRET HERE"
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=${PORT}

CMD ["./pong-transaction-service"]

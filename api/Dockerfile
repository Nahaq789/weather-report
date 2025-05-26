FROM rust:1.83.0 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY ./api ./api
COPY ./crates/sensor ./crates/sensor
COPY ./crates/sensor_consumer ./crates/sensor_consumer
COPY ./crates/sensor_simulator ./crates/sensor_simulator

RUN cargo build --release

RUN useradd -r -s /bin/false api

RUN chown api:api /app/target/release/api

USER api

WORKDIR /app

ENTRYPOINT ["./target/release/api"]
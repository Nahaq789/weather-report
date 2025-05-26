FROM rust:1.83.0 AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    librdkafka-dev \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY ./api ./api
COPY ./crates/sensor ./crates/sensor
COPY ./crates/sensor_consumer ./crates/sensor_consumer
COPY ./crates/sensor_simulator ./crates/sensor_simulator

RUN cargo build --release

RUN useradd -r -s /bin/false sensor_simulator

RUN chown sensor_simulator:sensor_simulator /app/target/release/sensor_simulator

USER sensor_simulator

WORKDIR /app

ENTRYPOINT ["./target/release/sensor_simulator"]

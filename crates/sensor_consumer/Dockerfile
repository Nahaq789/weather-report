FROM rust:1.83.0 AS build-stage

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    librdkafka-dev \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY api ./api
COPY crates/sensor ./crates/sensor
COPY crates/sensor_consumer ./creates/sensor_consumer

RUN cargo build --release

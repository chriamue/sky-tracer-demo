# syntax=docker/dockerfile:1.4
FROM rust:1.94-trixie AS builder

ARG TARGETPLATFORM

WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN --mount=type=cache,id=${TARGETPLATFORM}-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=${TARGETPLATFORM}-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=${TARGETPLATFORM}-target,target=/usr/src/app/target \
    cargo build --release -p orbital-beacon && \
    cp /usr/src/app/target/release/orbital-beacon /tmp/orbital-beacon

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tmp/orbital-beacon /usr/local/bin/orbital-beacon

EXPOSE 3002

CMD ["orbital-beacon"]

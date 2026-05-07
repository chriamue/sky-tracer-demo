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
    cargo build --release -p delay-orama && \
    cp /usr/src/app/target/release/delay-orama /tmp/delay-orama

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tmp/delay-orama /usr/local/bin/delay-orama

EXPOSE 3004

CMD ["delay-orama"]

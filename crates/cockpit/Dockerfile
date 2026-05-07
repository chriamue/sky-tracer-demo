# syntax=docker/dockerfile:1.4
FROM rust:1.94-trixie AS builder

ARG TARGETPLATFORM

RUN --mount=type=cache,id=${TARGETPLATFORM}-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=${TARGETPLATFORM}-git,target=/usr/local/cargo/git \
    cargo install trunk && \
    rustup target add wasm32-unknown-unknown && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

RUN --mount=type=cache,id=${TARGETPLATFORM}-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=${TARGETPLATFORM}-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=${TARGETPLATFORM}-target,target=/usr/src/app/target \
    cd crates/cockpit && \
    trunk build --release

FROM nginx:alpine

COPY --from=builder /usr/src/app/crates/cockpit/dist /usr/share/nginx/html
COPY crates/cockpit/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]

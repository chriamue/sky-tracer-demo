FROM rust:1.85 as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release -p airport-anywhere

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/airport-anywhere /usr/local/bin/airport-anywhere
COPY assets/airports.dat /usr/local/bin/assets/airports.dat

EXPOSE 3000

CMD ["airport-anywhere"]

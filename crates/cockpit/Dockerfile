FROM rust:1.85 as builder

# Install trunk and required dependencies
RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cd crates/cockpit && \
    trunk build --release

# Production stage
FROM nginx:alpine

# Copy the built assets
COPY --from=builder /usr/src/app/crates/cockpit/dist /usr/share/nginx/html

# Copy nginx configuration
COPY crates/cockpit/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

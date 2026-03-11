FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p approuter

FROM debian:bookworm-slim
RUN apt-get update -qq && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/approuter /usr/local/bin/
RUN mkdir -p /app/config
COPY --from=builder /app/approuter/config/registry.json /app/config/registry.json
EXPOSE 8080
ENV ROUTER_PORT=8080
ENV ROUTER_BIND=0.0.0.0
ENV ROUTER_CONFIG_DIR=/app
WORKDIR /app
CMD ["approuter", "--no-tunnel"]

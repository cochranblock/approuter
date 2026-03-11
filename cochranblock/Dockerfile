# Railway: build context = workspace root. Set RAILWAY_DOCKERFILE_PATH=cochranblock/Dockerfile.
FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p cochranblock --features approuter

FROM debian:bookworm-slim
RUN apt-get update -qq && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/cochranblock /usr/local/bin/
EXPOSE 8081
ENV PORT=8081
ENV BIND=0.0.0.0
# Set CB_BACKEND_URL in Railway: http://cochranblock.railway.internal:8081
# Set APPROUTER_URL: http://approuter.railway.internal:8080
CMD ["cochranblock"]

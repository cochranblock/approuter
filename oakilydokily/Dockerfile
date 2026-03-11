# Railway: build context = workspace root. Set RAILWAY_DOCKERFILE_PATH=oakilydokily/Dockerfile.
FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p oakilydokily --features approuter

FROM debian:bookworm-slim
RUN apt-get update -qq && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/oakilydokily /usr/local/bin/
EXPOSE 3000
ENV PORT=3000
ENV BIND=0.0.0.0
# Set APPROUTER_URL in Railway: http://approuter.railway.internal:8080
CMD ["oakilydokily"]

# Railway: build context = workspace root (parent of rogue-repo). Set RAILWAY_DOCKERFILE_PATH=rogue-repo/Dockerfile.
# Requires approuter and kova (for exopack path) as siblings. rogue-repo depends on ../../approuter.
FROM rust:1.83-bookworm AS builder
WORKDIR /app
COPY . .
WORKDIR /app/rogue-repo
RUN cargo build --release -p rogue-repo

FROM debian:bookworm-slim
RUN apt-get update -qq && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/rogue-repo/target/release/rogue-repo /usr/local/bin/
EXPOSE 3001
ENV PORT=3001
ENV BIND=0.0.0.0
# Set DATABASE_URL from Railway Postgres plugin
# Set APPROUTER_URL: http://approuter.railway.internal:8080
# Set REPO_BACKEND_URL: http://rogue-repo.railway.internal:3001
CMD ["rogue-repo"]

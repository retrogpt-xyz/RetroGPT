FROM rust@sha256:a45bf1f5d9af0a23b26703b3500d70af1abff7f984a7abef5a104b42c02a292b AS backend-builder

WORKDIR /app

# Copy workspace configuration and lock file first
COPY Cargo.toml Cargo.lock ./

# Create dummy crates structure to cache dependencies
RUN mkdir -p crates/rgpt crates/librgpt crates/libserver crates/rgpt-db
COPY crates/rgpt/Cargo.toml crates/rgpt/
COPY crates/librgpt/Cargo.toml crates/librgpt/

RUN mkdir -p crates/rgpt/src && \
  echo "fn main() {}" > crates/rgpt/src/main.rs && \
  mkdir -p crates/librgpt/src && \
  echo "pub fn dummy() {}" > crates/librgpt/src/lib.rs && \
  mkdir -p crates/rgpt-db/src && \
  echo "pub fn dummy() {}" > crates/rgpt-db/src/lib.rs && \
  mkdir -p crates/libserver/src && \
  echo "pub fn dummy() {}" > crates/libserver/src/lib.rs

COPY crates/rgpt-db/Cargo.toml crates/rgpt-db/Cargo.toml
COPY crates/libserver/Cargo.toml crates/libserver/Cargo.toml

RUN cargo build --release

RUN rm -rf crates/*/src

COPY crates/rgpt/src crates/rgpt/src
COPY crates/librgpt/src crates/librgpt/src
COPY crates/rgpt-db/src crates/rgpt-db/src
COPY crates/libserver/src crates/libserver/src

RUN touch crates/rgpt/src/main.rs && \
  touch crates/librgpt/src/lib.rs && \
  touch crates/libserver/src/lib.rs && \
  touch crates/rgpt-db/src/lib.rs 

RUN cargo build --release

FROM node@sha256:1745a99b66da41b5ccd6f7be3810f74ddab16eb4579de10de378adb50d2e6e6f AS frontend-builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm install

COPY frontend/ frontend/
COPY vite.config.ts tsconfig.json tsconfig.app.json tsconfig.node.json index.html ./
RUN npm run build

FROM rust@sha256:a45bf1f5d9af0a23b26703b3500d70af1abff7f984a7abef5a104b42c02a292b AS diesel-builder

RUN apt-get update && apt-get install -y \
  libssl-dev \
  libpq-dev \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:bookworm-slim@sha256:f70dc8d6a8b6a06824c92471a1a258030836b26b043881358b967bf73de7c5ab AS app

RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  libpq5 \
  pkg-config \
  libssl-dev \
  dumb-init \
  curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=backend-builder /app/target/release/rgpt .
COPY --from=frontend-builder /app/static/ static/
COPY --from=diesel-builder /usr/local/cargo/bin/diesel .
COPY migrations/ migrations/
COPY frontend/assets/favicon.ico static/
COPY diesel.toml .

EXPOSE 3000

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

CMD ["./rgpt"]

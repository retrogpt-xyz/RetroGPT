# syntax=docker/dockerfile:1.7-labs

FROM rust@sha256:532bc136da994ffe22cbc0a8df00c936d1a148d9fcb9202361987a4023696bf5 AS backend-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY --parents crates/*/Cargo.toml ./

RUN for dir in crates/*; do \
  mkdir $dir/src; \
  echo "pub fn dummy() {}" > "$dir/src/lib.rs"; \
  done

RUN cargo build --release

RUN rm -rf crates/*/src
COPY --parents crates/*/src ./

RUN touch crates/*/src/main.rs
RUN touch crates/*/src/lib.rs

RUN cargo build --release

FROM node@sha256:1745a99b66da41b5ccd6f7be3810f74ddab16eb4579de10de378adb50d2e6e6f AS frontend-builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm install

COPY frontend/ frontend/
COPY vite.config.ts tsconfig.json tsconfig.app.json tsconfig.node.json index.html ./
RUN npm run build

FROM rust@sha256:532bc136da994ffe22cbc0a8df00c936d1a148d9fcb9202361987a4023696bf5 AS diesel-builder

RUN apt-get update && apt-get install -y \
  libssl-dev \
  libpq-dev \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres


FROM debian:bookworm-slim@sha256:f70dc8d6a8b6a06824c92471a1a258030836b26b043881358b967bf73de7c5ab AS api

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

COPY --from=backend-builder /app/target/release/rgpt-api .
COPY --from=diesel-builder /usr/local/cargo/bin/diesel .

COPY migrations/ migrations/
COPY diesel.toml .

EXPOSE 4002

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

CMD ["./rgpt-api"]

FROM debian:bookworm-slim@sha256:f70dc8d6a8b6a06824c92471a1a258030836b26b043881358b967bf73de7c5ab AS static

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

COPY --from=backend-builder /app/target/release/rgpt-static .
COPY --from=frontend-builder /app/static/ static/

EXPOSE 4001

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

CMD ["./rgpt-static"]

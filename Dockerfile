FROM rust@sha256:a45bf1f5d9af0a23b26703b3500d70af1abff7f984a7abef5a104b42c02a292b AS backend-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir backend/ && \
  echo "fn main() {}" > backend/main.rs && \
  touch backend/lib.rs && \
  cargo build --release && \
  rm -rf backend

COPY backend/ backend/
RUN touch backend/main.rs && touch backend/lib.rs

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

FROM debian:bookworm-slim@sha256:f70dc8d6a8b6a06824c92471a1a258030836b26b043881358b967bf73de7c5ab as app

RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  libpq5 \
  dumb-init && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=backend-builder /app/target/release/retro_gpt_backend .
COPY --from=frontend-builder /app/static/ static/
COPY --from=diesel-builder /usr/local/cargo/bin/diesel .
COPY migrations/ migrations/
COPY diesel.toml .

EXPOSE 3000

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

CMD ["./retro_gpt_backend"]

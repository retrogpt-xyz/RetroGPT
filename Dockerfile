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

FROM archlinux@sha256:58fd363480dc61d0c657768605bca3c87d5b697cb8c2fe0217aad941c6a8a508 AS app

WORKDIR /app

RUN  pacman -Sy --noconfirm postgresql

COPY --from=backend-builder /app/target/release/retro_gpt_backend .
COPY --from=frontend-builder /app/static/ static/
COPY --from=diesel-builder /usr/local/cargo/bin/diesel .
COPY migrations/ migrations/
COPY diesel.toml .

EXPOSE 3000

CMD ["./retro_gpt_backend"]

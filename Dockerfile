FROM rust:latest AS backend-builder

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

FROM node:latest AS frontend-builder

WORKDIR /app

COPY . .
RUN npm install
RUN npm run build

FROM archlinux:latest AS app

WORKDIR /app

COPY --from=backend-builder /app/target/release/retro_gpt_backend .
COPY --from=frontend-builder /app/static/ static/
COPY serverconf.toml .

CMD ["./retro_gpt_backend"]

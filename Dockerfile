FROM rust:latest AS builder

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

FROM archlinux:latest AS app

WORKDIR /app

COPY --from=builder /app/target/release/retro_gpt_backend .
COPY static/ static/
COPY serverconf.toml .

CMD ["./retro_gpt_backend"]

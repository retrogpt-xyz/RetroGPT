FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM archlinux:latest AS app

WORKDIR /app

COPY --from=builder /app/target/release/retro_gpt_backend .

COPY static/ static/

COPY conf.toml .

CMD ["./retro_gpt_backend"]


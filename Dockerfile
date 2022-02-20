FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine

RUN apk add gcompat

WORKDIR /app

EXPOSE 8000

ENV TELEGRAM_TARGET_CHAT = ""
ENV TELEGRAM_BOT_TOKEN = ""
ENV ROCKET_ADDRESS = "0.0.0.0"
ENV POLL_RATE = "10"
ENV DEFAULT_TIMEOUT = "120"

COPY --from=builder /app/target/release/heartbeat .

CMD ["./heartbeat"]
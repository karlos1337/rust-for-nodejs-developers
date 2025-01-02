FROM rust:1.77.0

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]

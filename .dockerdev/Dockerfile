FROM rust:1.81.0

WORKDIR /app

COPY . .

RUN cargo install cargo-watch &&\
  cargo install sqlx-cli --no-default-features --features native-tls,postgres &&\
  apt-get update &&\
  apt-get install -y libpq-dev --no-install-recommends &&\
  apt-get clean &&\
  rm -rf /var/lib/apt/lists/*

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]

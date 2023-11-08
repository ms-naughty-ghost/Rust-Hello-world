# 開発環境
FROM rust:1.73.0 as develop
WORKDIR /api
COPY . /api
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
CMD [ "cargo", "watch", "-x", "run" ]

# ビルドステップ
FROM rust:1.73.0 as build
WORKDIR /api
COPY . /api
RUN cargo build --release --bin api

# 本番環境
FROM scratch as production
COPY --from=build /api/target/release/api /usr/local/bin/api
EXPOSE 8080
CMD ["/usr/local/bin/api"]
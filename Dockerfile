FROM rust:1.83-bullseye as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM rust:1.83-bullseye

COPY --from=builder /usr/src/app/target/release/rust-web-app-test /usr/local/bin/rust-web-app-test
EXPOSE 8080
CMD ["rust-web-app-test"]


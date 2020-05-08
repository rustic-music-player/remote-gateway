FROM rust:latest as builder

WORKDIR /build

COPY ["Cargo.toml", "Cargo.lock", "/build/"]
COPY src /build/src

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /build/target/release/ssh-proxy-api /ssh-proxy-api

ENTRYPOINT /ssh-proxy-api

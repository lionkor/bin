FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev

COPY . /sources
WORKDIR /sources
RUN cargo build --release

FROM scratch
COPY --from=builder /sources/target/release/bin /pastebin

EXPOSE 8000
ENTRYPOINT ["/pastebin", "0.0.0.0:8000", "--db-path", "/data/pastes.db"]

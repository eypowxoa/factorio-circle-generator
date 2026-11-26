FROM rust:1.100-alpine AS builder
WORKDIR /usr/src/factorio-circle-generator
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /usr/src/factorio-circle-generator/target/release/factorio-circle-generator /usr/local/bin/factorio-circle-generator
ENTRYPOINT ["factorio-circle-generator"]

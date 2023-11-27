FROM rust:1.74 as builder
COPY api /usr/src/doorsys/api
COPY protocol /usr/src/doorsys/protocol
WORKDIR /usr/src/doorsys/api
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/doorsys-api /usr/local/bin/doorsys-api
CMD ["doorsys-api"]

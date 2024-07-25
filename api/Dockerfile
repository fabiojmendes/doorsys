FROM rust:bookworm as builder
COPY api /usr/src/doorsys/api
COPY protocol /usr/src/doorsys/protocol
WORKDIR /usr/src/doorsys/api
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/doorsys-api /usr/local/bin/doorsys-api
COPY --from=builder /usr/src/doorsys/api/entrypoint.sh /
ENV DATABASE_HOST=localhost
ENV DATABASE_PORT=5432
ENV DATABASE_NAME=doorsys
ENV DATABASE_USER=root
ENV DATABASE_PASS=""
ENTRYPOINT ["/entrypoint.sh"]

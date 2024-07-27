FROM rust:bookworm AS builder
COPY api /usr/src/doorsys/api
COPY protocol /usr/src/doorsys/protocol
WORKDIR /usr/src/doorsys/api
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/doorsys-api /usr/local/bin/doorsys-api
COPY --from=builder /usr/src/doorsys/api/entrypoint.sh /

RUN apt-get update && \
  apt-get install -y curl && \
  apt-get clean 

ENV DATABASE_HOST=localhost
ENV DATABASE_PORT=5432
ENV DATABASE_NAME=doorsys
ENV DATABASE_USER=root
ENV DATABASE_PASS=""
ENV MQTT_HOST=localhost
ENV MQTT_PORT=1883
ENV MQTT_OPTS="client_id=doorsys-api&clean_session=false&keep_alive_secs=5"
ENV MQTT_USER=user
ENV MQTT_PASS=""
ENV RUST_LOG="doorsys_api=debug,tower_http=debug"

ENTRYPOINT ["/entrypoint.sh"]
CMD ["doorsys-api"]

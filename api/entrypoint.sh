#!/bin/bash

if [ -z "$DATABASE_URL" ]; then
  export DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASS}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}"
fi

if [ -z "$MQTT_URL" ]; then
  export MQTT_URL="mqtt://${MQTT_USER}:${MQTT_PASS}@${MQTT_HOST}:${MQTT_PORT}?${MQTT_OPTS}"
fi

export RUST_LOG

exec "$@"

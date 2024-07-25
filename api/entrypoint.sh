#!/bin/sh

if [ -z "$DATABASE_URL" ]; then
  DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASS}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}"
fi

doorsys-api "$@"

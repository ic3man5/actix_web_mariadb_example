#!/bin/bash
set -e
docker rm --force sqlx
docker run --name sqlx \
    -p 5432:5432 \
    -e POSTGRES_USER=user \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_DB=sqlx \
    -v $PWD/scripts/init.sql:/docker-entrypoint-initdb.d/init.sql \
    -d postgres:16-alpine
docker start sqlx
echo "Started docker container sqlx"
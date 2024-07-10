#!/bin/bash
set -e
docker rm --force sqlx
docker run --name sqlx \
    -p 3306:3306 \
    -e MARIADB_ROOT_USER=user \
    -e MARIADB_ROOT_PASSWORD=password \
    -e MARIADB_DATABASE=sqlx \
    -v $PWD/scripts/init.sql:/docker-entrypoint-initdb.d/init.sql \
    -d mariadb:11.4
docker start sqlx
echo "Started docker container sqlx"
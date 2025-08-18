#!/usr/bin/env bash


mkdir -pv /opt/postgres/test

podman run -d --name postgres \
    -e POSTGRES_PASSWORD=postgres \
    -v /opt/postgres/test/:/var/lib/postgresql/data \
    -p 5432:5432 \
    roux.io/postgres:latest
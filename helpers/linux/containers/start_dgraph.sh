#!/usr/bin/env bash


mkdir -pv /opt/dgraph/test/standalone


podman run --name dgraph -d -p "8080:8080" -p "9080:9080" -v /opt/dgraph/test/standalone:/dgraph roux.io/dgraph-standalone:latest

# podman run -d --name dgraph-zero \
#   -p 5080:5080 -p 6080:6080 \
#   -v /opt/dgraph/test/zero:/dgraph \
#   roux.io/dgraph:latest dgraph zero --my=zero:5080

# Start Dgraph Alpha
# podman run -d --name dgraph-alpha \
#   --link dgraph-zero:zero \
#   -p 8080:8080 -p 9080:9080 \
#   -v /opt/dgraph/test/alpha:/dgraph \
#   roux.io/dgraph:latest dgraph alpha \
#     --my=alpha:7080 --zero=zero:5080 --lru_mb=2048
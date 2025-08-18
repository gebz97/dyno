#!/usr/bin/env bash

podman run -d --name etcd-test \
   -p 2379:2379 -p 2380:2380 \
   -v /opt/etcd/test:/etcd-data \
   roux.io/etcd:coreos-latest   \
   etcd --data-dir=/etcd-data \
   --name test-node \
   --listen-client-urls=http://0.0.0.0:2379 \
   --advertise-client-urls=http://localhost:2379 \
   --listen-peer-urls=http://0.0.0.0:2380
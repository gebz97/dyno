#!/usr/bin/env bash

podman run -d --name rabbitmq-test \
    -p 5672:5672 -p 15672:15672 \
    -v /opt/rabbitmq/test:/var/lib/rabbitmq \
    roux.io/rabbitmq-management:alpine-latest
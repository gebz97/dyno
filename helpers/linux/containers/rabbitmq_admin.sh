#!/usr/bin/env bash

podman exec -it rabbitmq-test rabbitmqctl add_user admin admin
podman exec -it rabbitmq-test rabbitmqctl set_user_tags admin administrator
podman exec -it rabbitmq-test rabbitmqctl set_permissions -p / admin ".*" ".*" ".*"
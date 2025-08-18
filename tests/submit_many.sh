#!/bin/bash

for i in $(seq 1 10000); do
  wf_id="wf$i"
  json=$(cat <<EOF
{
  "id": "$wf_id",
  "tasks": [
    { "id": "A", "children": ["B","C"] },
    { "id": "B", "children": [] },
    { "id": "C", "children": [] }
  ]
}
EOF
)
  echo "$json" | amqp-publish -u amqp://admin:admin@gopher.gebz.local:5672/%2f -r dyno-submit-queue -p
done


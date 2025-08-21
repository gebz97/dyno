podman run -d \
  --name valkey \
  -p 6379:6379 \
  -v /opt/redis/rdb:/data \
  valkey/valkey:latest \
  --dir /data \
  --dbfilename dump.rdb

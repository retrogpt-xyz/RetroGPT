#!/bin/bash

set -ex

docker compose build
docker compose up -d --no-build
docker system prune -f --volumes > /dev/null 2>&1 &


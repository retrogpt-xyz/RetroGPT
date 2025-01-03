#!/bin/bash

DETACH_FLAG=""

while getopts "d" opt; do
  case $opt in
    d)
      DETACH_FLAG="-d"
      ;;
  esac
done

set -ex

docker compose build
docker compose up $DETACH_FLAG --no-build


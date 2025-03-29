#! /bin/bash

set -xe

docker buildx build -t retrogpt/rgpt_api:latest . --target api
docker buildx build -t retrogpt/rgpt_static:latest . --target static

docker push retrogpt/rgpt_api:latest
docker push retrogpt/rgpt_static:latest

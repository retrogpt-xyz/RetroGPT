set +xe

docker buildx build -t retrogpt/retrogpt:latest .

docker push retrogpt/retrogpt:latest

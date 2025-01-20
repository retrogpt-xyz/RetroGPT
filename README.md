# RetroGPT


### prod build:

runs over https

requires docker and docker compose

must have `./.env` file with `OPENAI_API_KEY` defined

must have https certificates from letsencrypt/certbot

run `./docker_rebuild.sh` to (re)build the containers and restart the containers if necessary

```bash
$ ./docker_rebuild.sh    # to run attached
$ ./docker_rebuild.sh -d # to run detached
```

### dev build:

runs on localhost port 3000

requires node and cargo

must have `OPENAI_API_KEY` defined either in environment vars or in `./.env` file

build the frontend:

```bash
$ npm install
$ npm run build
```

run the backend

```bash
$ cargo run
```

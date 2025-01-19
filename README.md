# RetroGPT

requires docker and docker compose

### prod build:

runs over https

must have `./.env` file with `OPENAI_API_KEY` defined
must have https certificates located in `certs/` dir: `./certs/cert.pem` and `./certs/key/pem`

run `./docker_rebuild.sh` to (re)build the containers and restart the containers if necessary

```bash
$ ./docker_rebuild.sh    # to run attached
$ ./docker_rebuild.sh -d # to run detached
```

### dev build:

runs on localhost port 3000

must have `OPENAI_API_KEY` defined in environment vars or in `./.env` file

build the frontend:

```bash
$ npm install
$ npm run build
```

run the backend

```bash
$ cargo run
```

# RetroGPT
### to run with docker:

requires docker and docker compose

runs on port 3000

must have `.env` file in project root with `OPENAI_API_KEY` defined

run `./docker_rebuild.sh` with privelege to the docker daemon

```bash
$ sudo ./docker_rebuild.sh
```

### to run locally

must have `OPENAI_API_KEY` environment variable defined

runs on port 3000

```bash
$ npm run build
$ cargo run --release
```

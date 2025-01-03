# RetroGPT
### to run with docker:

requires docker and docker compose

runs on port 3000

must have `.env` file in project root with `OPENAI_API_KEY` defined

run `./docker_rebuild.sh` with privelege to the docker daemon

```bash
$ ./docker_rebuild.sh    # to run attached
$ ./docker_rebuild.sh -d # to run detached
```

# RetroGPT

requires docker and docker compose

### prod build:

runs over https

must have `./.env` file with `OPENAI_API_KEY` defined

must have https certificates from certbot/letsencrypt

```bash
$ docker compose --profile prod up --build    # to run attached
                # or
$ docker compose --profile prod up --build -d # to run detached
```

### dev build:

runs over http

must have `./.env` file with `OPENAI_API_KEY` defined

```bash
$ docker compose --profile dev up --build    # to run attached
                # or
$ docker compose --profile dev up --build -d # to run detached
```

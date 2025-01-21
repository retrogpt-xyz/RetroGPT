# RetroGPT

builds with docker compose

### dev build:

runs over http

must have `.env` file with relevant keys defined

```bash
$ docker compose --profile dev up --build    # to run attached
                # or
$ docker compose --profile dev up --build -d # to run detached
```

### prod build:

runs over https

must have `.env` file with relevant keys defined

must have https certificates from certbot/letsencrypt

```bash
$ docker compose --profile prod up --build    # to run attached
                # or
$ docker compose --profile prod up --build -d # to run detached
```

### `.env` file:

must have the following values defined:

- `OPENAI_API_KEY`

prod build only:

- `HOSTNAME`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`
- `POSTGRES_DB`
- `DATABSE_URL`

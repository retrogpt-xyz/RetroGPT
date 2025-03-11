# RetroGPT

builds with docker compose

to build:

```bash
$ ./build.sh
```

the default profile is dev if none is provided

### dev build:

runs over http

must have `.env` file with relevant keys defined

### prod build:

runs over https

must have `.env` file with relevant keys defined

must have https certificates from certbot/letsencrypt

### `.env` file:

must have the following env vars defined:

- `OPENAI_API_KEY`
- `POSTGRES_DB`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`

additionally for prod build only:

- `HOSTNAME`

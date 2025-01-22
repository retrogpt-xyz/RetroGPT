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

must have the following values defined:

- `OPENAI_API_KEY`

prod build only:

- `HOSTNAME`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`
- `POSTGRES_DB`
- `DATABSE_URL`

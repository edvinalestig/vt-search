# vt_search

Run with `docker compose up --build` with the environment variable `DOCKER_BUILDKIT=1`.
This speeds up docker image rebuilds.

Run `docker compose up --build --wait` followed by `docker compose alpha watch` to enable live rebuilds.

The environment variables `VTClient` and `VTSecret` have to be set to your API keys.
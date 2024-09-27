# uphf-ics

uphf-ics is an app providing a GET endpoint available on `/ics` that returns, provided with an uphf cas username and password, your timetable in iCal format.

It is supposed to be used with calendar app that can sync from ics subscription.

## Running uphf-ics

### Using cargo

```bash
cargo run
```

#### Default cargo features

The `instrument` feature is enabled by default, to disable all default features use `--no-default-features`

### Using docker compose

#### Developement docker compose files

The docker compose files ending in `.dev.yml` are supposed to be used during development to build the app.

To run them, you can use:

```bash
docker compose -f docker/<name of docker compose file> up
```

#### Production docker compose files

These docker compose files (the ones ending in `.prod.yml`) are examples of how the app can be run in production. To use them, you shouldn't have to download the whole repository.

First, download a docker compose file from the repo (using `curl` or `wget` for example), rename it to `docker-compose.yml` and run it using:

```bash
docker compose up -d
```

#### Changing cargo features in docker

WIP

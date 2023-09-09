# TODO App

## Development

### Watch command

```shell
cargo watch -qc -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
cargo watch -qc -w src/ -x 'test model_todo_ -- --test-threads=1 --nocapture'
```

- q - quiet
- c - clear
- w - watch only source folder
- x - execute cargo command
- --nocapture - don't hide outputs from text executions

### Spin database

```shell
# Spin the database
podman run --rm -p5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg docker.io/postgres:14
# Database terminal
podman exec -it -u postgres pg psql
```
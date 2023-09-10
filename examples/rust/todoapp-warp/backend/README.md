# TODO App

## Development

### Watch command

```shell
# Dev app
cargo watch -qc -w src/ -x 'run -- ../frontend/web-folder'
# Test for model
cargo watch -qc -w src/ -x 'test model_ -- --test-threads=1 --nocapture'
# Test for web
cargo watch -qc -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
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
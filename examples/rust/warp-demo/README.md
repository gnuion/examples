
## Development

```sh
cargo watch -qc -w src/ -x run
```

- q - quiet
- c - clear
- w - watch only source folder
- x - execute cargo command


## Description

This is a simple app, which demmonstrates how to use warp filters. In the example we created two main routes, one for the api and one for static files. The api route is guarded by a security middleware. If security checks pass, the rest api responds. Finally we implemented a summy db_pool that should how we could continue linking our rest api to a postgresql database.
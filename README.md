# Crypto Issues

Scan crypto currency repositories for open issues.

To run locally first setup postgres database with
```sh
docker compose up -d
```

Then run migrations with:
```sh
diesel setup
diesel migration run
```

First seed the database with
```sh
cargo run --bin init
```

Then run api and cron with:
```sh
cargo run --bin api
```

To generate store run: 
```sh
sea generate entity --lib -o libs/store/src
```
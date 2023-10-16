# Backend

Backend for crypto issues is written in rust, using actix for api.

To run locally first setup postgres database with
```sh
docker compose up -d
```

Then run migrations with:
```sh
diesel setup
diesel migration run
```

First seed the database with:
```sh
cargo run --bin init
```

This should take about 1-2 hours

Then run api and cron with:
```sh
cargo run --bin api
```

To generate store run: 
```sh
sea generate entity -o libs/store/src/migrations --model-extra-derives Serialize,Deserialize --model-extra-attributes 'serde(rename_all = "camelCase")'
```
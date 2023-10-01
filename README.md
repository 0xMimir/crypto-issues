# Crypto Issues

Scan crypto currency repositories for open issues. 

To generate store run: 
```sh
sea generate entity --lib -o libs/store/src
```

## Tasks:
    * Create api for coingecko
      * fetch all assets
      * fetch all assets info
    * Create table to hold issue data.
    * Create cron to scan over repositores for issues.
    * Create cron to check if issue has been closed.
    * Create api routes
    * Create openapi docs
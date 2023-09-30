create table cryptocurrencies(
    id uuid primary key default uuid_generate_v4() not null,
    name varchar(255) not null unique,
    coingecko_id varchar(255) not null unique,
    github varchar(255) unique,
    gitlab varchar(255) unique,
    description text
)
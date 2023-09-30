create table cryptocurrencies(
    id uuid primary key default uuid_generate_v4() not null,
    name varchar(255) not null,
    coingecko_id varchar(255) not null,
    github varchar(255),
    gitlab varchar(255),
    description text
)
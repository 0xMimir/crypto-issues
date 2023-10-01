create table github_projects(
    id uuid primary key default uuid_generate_v4() not null,
    name varchar(255) not null unique
);

create table cryptocurrencies(
    id uuid primary key default uuid_generate_v4() not null,
    name varchar(255) not null unique,
    coingecko_id varchar(255) not null unique,
    github uuid references github_projects(id),
    gitlab varchar(255),
    description text
)
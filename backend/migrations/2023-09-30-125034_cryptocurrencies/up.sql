create table github_projects(
    id uuid primary key default uuid_generate_v4() not null,
    name varchar(255) not null unique,
    profile_type varchar(20),
    url text,
    followers bigint not null default 0
);

create table cryptocurrencies(
    id uuid primary key default uuid_generate_v4() not null,
    name varchar(255) not null unique,
    coingecko_id varchar(255) not null unique,
    github uuid references github_projects(id) on delete set null,
    gitlab varchar(255),
    description text
)
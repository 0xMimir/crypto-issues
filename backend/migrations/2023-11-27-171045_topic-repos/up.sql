create table topics_repositories(
    id uuid primary key default uuid_generate_v4() not null,
    repository_name varchar(255) not null,
    repository_owner varchar(255) not null,
    language varchar(255),
    stargazers_count bigint not null,
    scraped boolean default false,
    unique(repository_name, repository_owner)
)
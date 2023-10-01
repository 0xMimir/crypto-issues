create table github_repositories(
    id uuid primary key default uuid_generate_v4() not null,
    project  varchar(255) references cryptocurrencies(github) not null,
    repository_name varchar(255) not null,
    unique(project, repository_name)
)
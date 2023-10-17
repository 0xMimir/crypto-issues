create table github_repositories(
    id uuid primary key default uuid_generate_v4() not null,
    project uuid not null references github_projects(id) on delete cascade,
    repository_name varchar(255) not null,
    language varchar(255),
    stargazers_count bigint not null,
    forks_count bigint not null,
    created_at timestamp not null,
    updated_at timestamp not null,
    archived boolean not null,
    unique(project, repository_name)
)
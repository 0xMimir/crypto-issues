create table issues(
    id uuid primary key default uuid_generate_v4() not null,
    repository uuid references github_repositories(id) not null,
    issue bigint not null,
    title text not null,
    description text,
    created_at timestamp not null,
    closed boolean not null,
    unique(repository, issue)
)
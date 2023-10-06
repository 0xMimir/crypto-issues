create table issues(
    id uuid primary key default uuid_generate_v4() not null,
    repository uuid not null references github_repositories(id) on delete cascade,
    issue bigint not null,
    title text not null,
    description text,
    created_at timestamp not null,
    closed boolean not null,
    unique(repository, issue)
)
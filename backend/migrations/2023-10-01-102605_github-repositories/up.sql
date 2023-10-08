create table github_repositories(
    id uuid primary key default uuid_generate_v4() not null,
    project uuid not null references github_projects(id) on delete cascade,
    repository_name varchar(255) not null,
    unique(project, repository_name)
)
create table github_repositories(
    id uuid primary key default uuid_generate_v4() not null,
    project uuid references github_projects(id) not null,
    repository_name varchar(255) not null,
    unique(project, repository_name)
)
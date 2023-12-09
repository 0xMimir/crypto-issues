create table issue_labels(
    id uuid primary key default uuid_generate_v4() not null,
    issue_id uuid not null references issues(id) on delete cascade,
    name text not null
)
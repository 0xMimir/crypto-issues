use chrono::NaiveDateTime;
use sea_orm::Set;
use store::github_repositories;
use uuid::Uuid;

pub fn default_github_repo(id: Uuid) -> github_repositories::ActiveModel {
    github_repositories::ActiveModel {
        project: Set(id),
        repository_name: Set("good-repo".to_owned()),
        stargazers_count: Set(0),
        forks_count: Set(0),
        id: Default::default(),
        language: Set(None),
        created_at: Set(NaiveDateTime::default()),
        updated_at: Set(NaiveDateTime::default()),
        archived: Set(false),
        fork: Set(false),
    }
}

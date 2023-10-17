//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "github_repositories")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub project: Uuid,
    pub repository_name: String,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived: bool,
    pub fork: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::github_projects::Entity",
        from = "Column::Project",
        to = "super::github_projects::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    GithubProjects,
    #[sea_orm(has_many = "super::issues::Entity")]
    Issues,
}

impl Related<super::github_projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GithubProjects.def()
    }
}

impl Related<super::issues::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Issues.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

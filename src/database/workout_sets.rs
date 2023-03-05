//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use super::sea_orm_active_enums::Bodypart;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "workout_sets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub date: DateTimeWithTimeZone,
    pub exercise_name: String,
    pub category: Bodypart,
    pub reps: i32,
    #[sea_orm(column_type = "Double")]
    pub weight: f64,
    pub comment: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

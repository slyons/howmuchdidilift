use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use loco_rs::{
    validation,
    validator::Validate
};
use sea_orm::sea_query::{Expr, Func};
use sea_orm::{entity::prelude::*, ActiveValue, DatabaseConnection, DbErr, TransactionTrait};
use loco_rs::model::{ModelError, ModelResult};
use rand::Rng;
use sea_orm::QuerySelect;
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Deserialize};
use interface::MeasureCreate;

pub use super::_entities::measures::{self, ActiveModel, Entity, Model};

#[derive(Debug, Validate, Deserialize)]
pub struct ModelValidator {
    #[validate(length(min=2, message="Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(range(min=0))]
    pub grams: f64
}

impl From<&ActiveModel> for ModelValidator {
    fn from(value: &ActiveModel) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
            grams: *value.grams.as_ref()
        }
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::measures::ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr> where C: ConnectionTrait {
        {
            self.validate()?;
            Ok(self)
        }
    }
}

impl super::_entities::measures::Model {
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> ModelResult<Self> {
        let measure = measures::Entity::find()
            .filter(Expr::expr(Func::lower(Expr::col(measures::Column::Name))).ilike(name))
            .one(db)
            .await?;
        measure.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_random(db: &DatabaseConnection) -> ModelResult<Self> {
        let count = measures::Entity::find()
            .count(db)
            .await?;
        let random_offset = rand::thread_rng().gen_range(0..count);
        let measure = measures::Entity::find()
            .offset(random_offset)
            .one(db)
            .await?;
        measure.ok_or_else(|| ModelError::EntityNotFound)
    }
}

impl super::_entities::measures::ActiveModel {
    pub async fn create(db: &DatabaseConnection, params: MeasureCreate) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if measures::Entity::find()
            .filter(Expr::expr(Func::lower(Expr::col(measures::Column::Name))).ilike(&params.name))
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        let measure = measures::ActiveModel {
            name: ActiveValue::Set(params.name),
            name_plural: ActiveValue::Set(params.name_plural),
            grams: ActiveValue::Set(params.grams),
            ..Default::default()
        }
            .insert(&txn)
            .await?;

        txn.commit().await?;

        Ok(measure.into())
    }

    pub fn validate(&self) -> Result<(), DbErr> {
        let validator: ModelValidator = self.into();
        validator
            .validate()
            .map_err(|e| validation::into_db_error(&e))
    }
}

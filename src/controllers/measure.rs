#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum_macros::debug_handler;

use interface::{InputWeightType, MeasureCreate, RandomWeightRequest, RandomWeightResponse};
use loco_rs::prelude::*;
use sea_orm::{prelude::DateTimeUtc, TryIntoModel};

use crate::models::{
    _entities::measures::{ActiveModel, Entity, Model},
    users,
};

async fn load_item(auth: auth::JWT, ctx: &AppContext, id: i32) -> Result<Model> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<MeasureCreate>,
) -> Result<Json<Model>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(
        ActiveModel::create(&ctx.db, params)
            .await?
            .try_into_model()?,
    )
}

pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<MeasureCreate>,
) -> Result<Json<Model>> {
    let item = load_item(auth, &ctx, id).await?;
    let mut item = item.into_active_model();
    item.name = Set(params.name);
    item.updated_at = Set(DateTimeUtc::default().naive_local());
    item.grams = Set(params.grams);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<()> {
    load_item(auth, &ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<Model>> {
    format::json(load_item(auth, &ctx, id).await?)
}

#[debug_handler]
pub async fn convert(
    State(ctx): State<AppContext>,
    Json(params): Json<RandomWeightRequest>,
) -> Result<Json<RandomWeightResponse>> {
    let source_grams = params.input_amt
        * match params.input_type {
            InputWeightType::Lbs => 453.592,
            InputWeightType::Kgs => 1000.0,
        };

    let rnd_measure = Model::find_random(&ctx.db).await?;
    let div = source_grams / rnd_measure.grams;
    let count_str = if div > 1.0 {
        rnd_measure.name_plural
    } else {
        rnd_measure.name
    };
    let div = (div * 100.0).round() / 100.0;
    //let frac = FuzzyFraction::from_ints(source_grams, rnd_measure.grams);

    Ok(Json(RandomWeightResponse {
        input_amt: params.input_amt,
        input_type: params.input_type,
        output_weight: format!("{} {}", div, count_str),
        //output_weight: format!("{} => {} / {} = {} {}", params.input_amt, source_grams,
        // rnd_measure.grams, div, count_str),
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("measures")
        .add("/", get(list))
        .add("/", post(add))
        .add("/convert", post(convert))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}

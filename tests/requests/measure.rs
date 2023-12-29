use axum::Json;
use insta::{assert_debug_snapshot, with_settings};
use loco_rs::prelude::AppContext;
use interface::{InputWeightType, Measure, MeasureCreate, RandomWeightRequest, RandomWeightResponse};
use liftcalc::app::App;
use loco_rs::{testing, TestServer};
use serial_test::serial;

use crate::requests::prepare_data;
use crate::requests::prepare_data::{auth_header, init_user_login};

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("measure_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_create() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        //testing::seed::<App>(&ctx.db).await.unwrap();
        let measure = prepare_data::create_measure(&request, &ctx).await;

        with_settings!({
            filters => {
                 let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                    combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                    combined_filters
            }
        }, {
            assert_debug_snapshot!(
                measure
            );
        });
    })
    .await;
}

async fn list_measures(request:&TestServer, ctx: &AppContext) -> Vec<Measure> {
    let user = prepare_data::init_user_login(request, ctx).await;
    let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
    let measures = request.get("/api/measures")
        .add_header(auth_key, auth_value)
        .await;
    measures.assert_status_ok();
    measures.json()
}

#[tokio::test]
#[serial]
async fn can_list() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let empty_measures = list_measures(&request, &ctx).await;

        let measure = prepare_data::create_measure(&request, &ctx).await;

        let measures = list_measures(&request, &ctx).await;
        assert_debug_snapshot!((empty_measures, measures))
    }).await
}

#[tokio::test]
#[serial]
async fn can_delete() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let measure = prepare_data::create_measure(&request, &ctx).await;

        let all_measures = list_measures(&request, &ctx).await;
        assert!(!all_measures.is_empty());
        let measure_id = all_measures[0].id;

        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let measures = request.delete(&format!("/api/measures/{}", measure_id))
            .add_header(auth_key, auth_value)
            .await;
        measures.assert_status_ok();

        let measures = list_measures(&request, &ctx).await;
        assert_debug_snapshot!((all_measures, measures))
    }).await
}

#[tokio::test]
#[serial]
async fn can_update() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let measure = prepare_data::create_measure(&request, &ctx).await;

        let mut all_measures = list_measures(&request, &ctx).await;
        assert!(!all_measures.is_empty());

        let mut measure = all_measures.pop().unwrap();
        measure.grams = measure.grams + 2.0;
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
        let measures = request.post(&format!("/api/measures/{}", measure.id))
            .json(&measure)
            .add_header(auth_key.clone(), auth_value.clone())
            .await;
        measures.assert_status_ok();

        let updated_measure = request.get(&format!("/api/measures/{}", measure.id))
            .add_header(auth_key, auth_value)
            .await
            .json::<Measure>();
        let list = list_measures(&request, &ctx).await;

        assert_eq!(measure.grams, updated_measure.grams);
        assert_debug_snapshot!((measure, updated_measure))
    }).await
}

#[tokio::test]
#[serial]
async fn can_convert_lbs() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let measure = prepare_data::create_measure(&request, &ctx).await;

        let user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&user.token);

        let convert_request = RandomWeightRequest {
            input_amt: 100.0,
            input_type: InputWeightType::Lbs,
        };
        let random_weight = request
            .post("/api/measures/convert")
            .add_header(auth_key, auth_value)
            .json(&convert_request)
            .await;
        random_weight.assert_status_ok();

        let random_weight: RandomWeightResponse = random_weight.json();
        assert_debug_snapshot!(random_weight);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_convert_kgs() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let measure = prepare_data::create_measure(&request, &ctx).await;

        let user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&user.token);

        let convert_request = RandomWeightRequest {
            input_amt: 1.0,
            input_type: InputWeightType::Kgs,
        };
        let random_weight = request
            .post("/api/measures/convert")
            .add_header(auth_key, auth_value)
            .json(&convert_request)
            .await;
        random_weight.assert_status_ok();

        let random_weight: RandomWeightResponse = random_weight.json();
        assert_debug_snapshot!(random_weight);
    })
    .await;
}

use axum::Json;
use insta::{assert_debug_snapshot, with_settings};
use interface::{InputWeightType, MeasureCreate, RandomWeightRequest, RandomWeightResponse};
use liftcalc::app::App;
use loco_rs::testing;
use serial_test::serial;

use crate::requests::prepare_data;

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
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        let create_request = MeasureCreate {
            name: "cheeseburger".to_string(),
            name_plural: "cheeseburgers".to_string(),
            grams: 500.0,
        };
        let measures = request
            .post("/api/measures")
            .add_header(auth_key, auth_value)
            .json(&create_request)
            .await;

        with_settings!({
            filters => {
                 let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                    combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                    combined_filters
            }
        }, {
            assert_debug_snapshot!(
                (measures.status_code(), measures.text())
            );
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_convert_lbs() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        let create_request = MeasureCreate {
            name: "gramburger".to_string(),
            name_plural: "gramburgers".to_string(),
            grams: 500.0,
        };
        let measures = request
            .post("/api/measures")
            .add_header(auth_key, auth_value)
            .json(&create_request)
            .await;
        measures.assert_status_ok();

        let convert_request = RandomWeightRequest {
            input_amt: 100.0,
            input_type: InputWeightType::Lbs,
        };
        let random_weight = request
            .post("/api/measures/convert")
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
        let user = prepare_data::init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = prepare_data::auth_header(&user.token);

        let create_request = MeasureCreate {
            name: "gramburger".to_string(),
            name_plural: "gramburgers".to_string(),
            grams: 1.0,
        };
        let measures = request
            .post("/api/measures")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&create_request)
            .await;
        measures.assert_status_ok();

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

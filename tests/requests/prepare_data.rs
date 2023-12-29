use axum::http::{HeaderName, HeaderValue};
use interface::{LoginResponse, MeasureCreate};
use liftcalc::models::users;
use loco_rs::{app::AppContext, TestServer};
use crate::requests::prepare_data;

const USER_EMAIL: &str = "test@loco.com";
const USER_PASSWORD: &str = "1234";

pub struct LoggedInUser {
    pub user: users::Model,
    pub token: String,
}

pub async fn init_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let register_payload = serde_json::json!({
        "name": "loco",
        "email": USER_EMAIL,
        "password": USER_PASSWORD,
        "password_confirm": USER_PASSWORD,
    });

    //Creating a new user
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;
    let user = users::Model::find_by_email(&ctx.db, USER_EMAIL)
        .await
        .unwrap();

    let verify_payload = serde_json::json!({
        "token": user.email_verification_token,
    });

    request.post("/api/auth/verify").json(&verify_payload).await;

    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
        token: login_response.token,
    }
}

pub async fn create_measure(request: &TestServer, ctx: &AppContext) -> interface::Measure {
    let user = init_user_login(&request, &ctx).await;
    let (auth_key, auth_value) = auth_header(&user.token);

    let create_request = interface::MeasureCreate {
        name: "gram".to_string(),
        name_plural: "grams".to_string(),
        grams: 1.0,
    };

    let measures = request
        .post("/api/measures")
        .add_header(auth_key, auth_value)
        .json(&create_request)
        .await;
    measures.assert_status_ok();
    measures.json()
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (HeaderName::from_static("authorization"), auth_header_value)
}

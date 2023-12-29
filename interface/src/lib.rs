use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasureCreate {
    pub name: String,
    pub name_plural: String,
    pub grams: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Measure {
    pub id: i32,
    pub name: String,
    pub name_plural: String,
    pub grams: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, EnumIter, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum InputWeightType {
    Lbs,
    Kgs,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomWeightRequest {
    pub input_amt: f64,
    pub input_type: InputWeightType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RandomWeightResponse {
    pub when: DateTime<Utc>,
    pub input_amt: f64,
    pub input_type: InputWeightType,
    pub output_weight: String,
}



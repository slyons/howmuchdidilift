use serde::{Serialize, Deserialize};
use strum::EnumIter;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub name_plural:  String,
    pub grams: f64
}

#[derive(Debug, Deserialize, Serialize, EnumIter)]
#[serde(rename_all="lowercase")]
pub enum InputWeightType {
    Lbs,
    Kgs
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomWeightRequest {
    pub input_amt: f64,
    pub input_type: InputWeightType
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomWeightResponse {
    pub input_amt: f64,
    pub input_type: InputWeightType,
    pub output_weight: String
}
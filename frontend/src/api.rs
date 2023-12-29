use interface::*;

use gloo_net::http::{Request, RequestBuilder};
use eyre::{WrapErr, Result};
use leptos::server_fn::serde;

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str
}

#[derive(Clone)]
pub struct AuthorizedApi {
    pub url: &'static str,
    pub token: String
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self {url}
    }

    pub async fn register(&self, params: &RegisterParams) -> Result<()> {
        let url = format!("{}/auth/register", self.url);
        let response = Request::post(&url).json(params)?.send().await?;
        response.json::<()>()
            .await
            .wrap_err(format!("Registration call failed"))
    }

    pub async fn login(&self, params: &LoginParams) -> Result<AuthorizedApi> {
        let url = format!("{}/auth/login", self.url);
        let response = Request::post(&url)
            .json(params)?
            .send()
            .await?;
        let login_response:LoginResponse = response.json().await?;
        Ok(AuthorizedApi::new(self.url, login_response.token))
    }

    pub async fn convert(&self, params: RandomWeightRequest) -> Result<RandomWeightResponse> {
        let url = format!("{}/measures/convert", self.url);
        let response = Request::post(&url)
            .json(&params)?
            .send()
            .await?;
        Ok(response.json().await?)
    }
}

impl AuthorizedApi {
    pub fn new(url: &'static str, token: String) -> Self {
        Self {url, token }
    }

    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token)
    }

    fn with_auth(&self, rb: RequestBuilder) -> RequestBuilder {
        rb.header("Authorization", &self.auth_header_value())
    }

    async fn send<T>(&self, mut req:RequestBuilder) -> Result<T>
    where T: serde::de::DeserializeOwned {
        let response = req
            .header("Authorization", &self.auth_header_value())
            .send()
            .await?;
        response.json::<T>()
            .await
            .map_err(|e|e.into())
    }

    pub async fn current_user(&self) -> Result<CurrentResponse> {
        let url = format!("{}/user/current", self.url);
        self.with_auth(Request::get(&url))
            .send()
            .await?
            .json::<CurrentResponse>()
            .await
            .wrap_err(format!("Failed to fetch current user"))
    }

    pub async fn add(&self, params: MeasureCreate) -> Result<Measure> {
        let url = format!("{}/measures", self.url);
        self.with_auth(Request::post(&url))
            .json(&params)?
            .send()
            .await?
            .json::<Measure>()
            .await
            .map_err(|e| e.into())
    }

    pub async fn list(&self) -> Result<Vec<Measure>> {
        let url = format!("{}/measures", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn get_one(&self, id: i32) -> Result<Measure> {
        let url = format!("{}/measures/{}", self.url, id);
        self.send(Request::get(&url)).await
    }

    pub async fn delete_one(&self, id: i32) -> Result<()> {
        let url = format!("{}/measures/{}", self.url, id);
        self.send(Request::delete(&url)).await
    }

    pub async fn update_one(&self, measure: Measure) -> Result<Measure> {
        let url = format!("{}/measures/{}", self.url, measure.id);
        self.with_auth(Request::post(&url))
            .json(&measure)?
            .send().await?
            .json::<Measure>()
            .await
            .wrap_err(format!("Failed to update measure"))
    }
}
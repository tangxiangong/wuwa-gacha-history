mod request;
mod response;
mod utils;

pub use request::*;
pub use response::*;
pub use utils::*;

use crate::{Error, Result};
use reqwest::{
    Client, ClientBuilder,
    header::{HeaderMap, HeaderValue, ACCEPT, ORIGIN, REFERER, USER_AGENT},
};

const API_URL: &str = "https://gmserver-api.aki-game2.com/gacha/record/query";
const RESOURCES_ORIGIN: &str = "https://aki-gm-resources.aki-game.com";
const RESOURCES_REFERER: &str = "https://aki-gm-resources.aki-game.com/";
const WEBVIEW_UA: &str = "Mozilla/5.0 (iPhone; CPU OS 18_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148";

#[derive(Debug, Clone)]
pub struct GachaHistoryClient {
    params: RequestParams,
    client: Client,
}

impl GachaHistoryClient {
    pub fn new(params: RequestParams) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
        headers.insert(ORIGIN, HeaderValue::from_static(RESOURCES_ORIGIN));
        headers.insert(REFERER, HeaderValue::from_static(RESOURCES_REFERER));
        headers.insert(USER_AGENT, HeaderValue::from_static(WEBVIEW_UA));
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(Self { params, client })
    }

    pub async fn fetch_all(&mut self, pool_type: CardPool) -> Result<Vec<ResponseRecord>> {
        self.params.card_pool_type = pool_type;
        self.params.card_pool_id.clear();
        let response = self.send().await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.message,
            });
        }
        Ok(response.data)
    }

    async fn send(&self) -> Result<GachaHistoryResponse> {
        let accept_language = HeaderValue::from_str(&self.params.language_code)
            .unwrap_or_else(|_| HeaderValue::from_static("zh-Hans"));
        let response = self
            .client
            .post(API_URL)
            .header(reqwest::header::ACCEPT_LANGUAGE, accept_language)
            .json(&self.params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

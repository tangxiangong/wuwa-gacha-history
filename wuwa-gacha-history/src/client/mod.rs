mod request;
mod response;
mod utils;

pub use request::*;
pub use response::*;
pub use utils::*;

use crate::{Error, Result};
use reqwest::{Client, ClientBuilder};

const API_URL: &str = "https://gmserver-api.aki-game2.com/gacha/record/query";

#[derive(Debug, Clone)]
pub struct GachaHistoryClient {
    params: RequestParams,
    client: Client,
}

impl GachaHistoryClient {
    pub fn new(params: RequestParams) -> Result<Self> {
        let client = ClientBuilder::new().build()?;
        Ok(Self { params, client })
    }

    pub async fn fetch_all(&mut self, pool_type: CardPool) -> Result<Vec<ResponseRecord>> {
        self.params.card_pool_type = pool_type;
        if let Some(pool_id) = pool_type.pool_id() {
            self.params.card_pool_id = pool_id;
        }
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
        let response = self
            .client
            .post(API_URL)
            .json(&self.params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

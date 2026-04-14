mod request;
mod response;
mod utils;

pub use request::*;
pub use response::*;
pub use utils::*;

use crate::{Error, Result};
use reqwest::{Client, ClientBuilder};

const API_URL: &str = "https://aki-game2.com/gacha/record/query";

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

    pub async fn fetch(
        &mut self,
        pool_type: CardPool,
        last_id: Option<String>,
    ) -> Result<(Option<String>, Vec<ResponseRecord>)> {
        self.params.size = 20;
        self.params.last_id = last_id;
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
        let last_id = response.data.last().map(|record| record.id.clone());
        Ok((last_id, response.data))
    }

    pub async fn fetch_all(&mut self, pool_type: CardPool) -> Result<Vec<ResponseRecord>> {
        let mut all_records = Vec::new();
        let mut last_id = None;

        loop {
            let (new_last_id, records) = self.fetch(pool_type, last_id).await?;
            let count = records.len();
            all_records.extend(records);

            if count < 20 {
                break;
            }

            last_id = new_last_id;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        Ok(all_records)
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

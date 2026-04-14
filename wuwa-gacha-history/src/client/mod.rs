mod request;
mod response;
mod utils;

pub use request::*;
pub use response::*;
pub use utils::*;

use reqwest::{Client, ClientBuilder};

const API_URL: &str = "https://aki-game2.com/gacha/record/query";

#[derive(Debug, Clone)]
pub struct GachaHistoryClient {
    params: RequestParams,
    client: Client,
}

impl GachaHistoryClient {
    pub fn new(params: RequestParams) -> Self {
        let client = ClientBuilder::new()
            .build()
            .expect("Failed to build client.");
        Self { params, client }
    }

    pub async fn fetch(
        &mut self,
        pool_type: CardPool,
        last_id: Option<String>,
    ) -> (Option<String>, Vec<ResponseRecord>) {
        self.params.size = 20;
        self.params.last_id = last_id;
        if !matches!(
            pool_type,
            CardPool::FeaturedResonatorConvene | CardPool::FeaturedWeaponConvene
        ) {
            self.params.card_pool_id = pool_type.pool_id().unwrap();
        }
        let response = self.send().await;
        let last_id = response.data.last().map(|record| record.id.clone());
        (last_id, response.data)
    }

    pub async fn fetch_all(&mut self, pool_type: CardPool) -> Vec<ResponseRecord> {
        let (mut last_id, mut records) = self.fetch(pool_type, None).await;

        while records.len() < 20 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let (new_last_id, new_records) = self.fetch(pool_type, last_id).await;
            last_id = new_last_id;
            records.extend(new_records);
        }

        records
    }

    async fn send(&self) -> GachaHistoryResponse {
        self.client
            .post(API_URL)
            .json(&self.params)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

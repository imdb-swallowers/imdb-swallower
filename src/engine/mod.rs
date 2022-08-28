use std::error::Error;

use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use scraper::Html;
use urlencoding::encode;

use crate::search::By;

#[derive(Debug, Clone)]
pub struct ImdbSearchEngine {
    hyper_client: Client<HttpsConnector<HttpConnector>>,
    base_uri: String,
}

impl ImdbSearchEngine {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let hyper_client = Client::builder().build::<_, hyper::Body>(https);
        let base_uri = "https://www.imdb.com".to_string();
        Self {
            hyper_client,
            base_uri,
        }
    }

    pub fn base_uri(&self) -> &str {
        self.base_uri.as_ref()
    }

    pub async fn search_by<B: By>(
        &self,
        by: B,
        query: &str,
    ) -> Result<B::ParseResult, Box<dyn Error + Send + Sync>> {
        let encoded_query = encode(query).to_string();
        let uri = by.get_uri(self, &encoded_query);
        let mut resp = self.hyper_client.get(uri).await?;
        Ok(by.parse_result(Html::parse_document(&String::from_utf8(
            hyper::body::to_bytes(resp.body_mut())
                .await?
                .into_iter()
                .collect(),
        )?)))
    }
}

mod types;

use eyre::Result;
use http::Uri;
pub use types::*;

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: reqwest::Client::builder()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()?,
        })
    }

    pub async fn g(&self, id: u32) -> Result<Gallery> {
        let res = self
            .inner
            .get(
                Uri::builder()
                    .scheme("https")
                    .authority("nhentai.net")
                    .path_and_query(format!("/api/gallery/{}", id))
                    .build()?
                    .to_string(),
            )
            .send()
            .await?;

        serde_json::from_str(res.text().await?.as_str()).map_err(From::from)
    }

    pub async fn i(&self, url: &Uri) -> Result<Vec<u8>> {
        Ok(self
            .inner
            .get(url.to_string())
            .send()
            .await?
            .bytes()
            .await?
            .into())
    }
}

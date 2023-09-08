mod types;

use eyre::Result;
use http::Uri;
pub use types::*;

#[derive(Debug, Clone)]
pub struct Client {
    inner: ureq::Agent,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: ureq::AgentBuilder::new()
                .user_agent(&format!(
                    "{}/{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ))
                .build(),
        }
    }

    pub fn g(&self, id: u32) -> Result<Gallery> {
        let res = self
            .inner
            .get(
                &Uri::builder()
                    .scheme("https")
                    .authority("nhentai.net")
                    .path_and_query(format!("/api/gallery/{id}"))
                    .build()?
                    .to_string(),
            )
            .call()?;

        Ok(res.into_json()?)
    }

    pub fn i(&self, url: &Uri) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.inner
            .get(&url.to_string())
            .call()?
            .into_reader()
            .read_to_end(&mut buf)?;

        Ok(buf)
    }
}

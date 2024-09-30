use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::fs;
// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;
    async fn fetch(&self) -> Result<String, Self::Error> {
        let response = reqwest::get(self.0).await?;
        let text = response.text().await?;
        Ok(text)
    }
}
#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;
    async fn fetch(&self) -> Result<String, Self::Error> {
        let contents = fs::read_to_string(&self.0[7..]).await?;
        Ok(contents)
    }
}

pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        "http" => UrlFetcher(name).fetch().await,
        "file" => FileFetcher(name).fetch().await,
        _ => return Err(anyhow!("unsupported source")),
    }
}

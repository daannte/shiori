use crate::client::HTTP_CLIENT;
use scraper::Html;

pub async fn fetch_doc(url: &str) -> Result<Html, reqwest::Error> {
    let body = HTTP_CLIENT
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(Html::parse_document(&body))
}

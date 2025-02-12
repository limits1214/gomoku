use serde_json::{json, Value};

use crate::config::APP_CONFIG;

pub async fn ws_initial(
    connection_id: String,
    http_client: reqwest::Client,
    jwt: Option<String>,
) -> anyhow::Result<()> {
    // get jwt by connection_id
    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/");
    let res = http_client
        .post(url)
        .json(&json!({
            "connection_id": connection_id
        }))
        .send()
        .await?;

    let j = res.json::<Value>().await?;
    let a = j.get("index").unwrap().get("index").unwrap();

    Ok(())
}

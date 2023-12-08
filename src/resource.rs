use crate::config::*;
use reqwest::Client;
use std::error::Error;

/// Execute a select query on the db
pub async fn select(req: &str) -> Result<String, Box<dyn Error>> {
    let resp = Client::new()
        .post(TOYDB)
        .body(req.to_string())
        .send()
        .await?
        .text()
        .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SELECT1: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends"}}"#;
    const TEST_SELECT2: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends", "filter": {"Eq": [{ "Col": "Age"}, {"Val": {"Integer": 18}}]}}}"#;

    #[tokio::test]
    async fn test_select() {
        let s = select(TEST_SELECT1).await.unwrap();
        println!("s1: {s:#?}");

        let s = select(TEST_SELECT2).await.unwrap();
        println!("s2: {s:#?}");
    }
}

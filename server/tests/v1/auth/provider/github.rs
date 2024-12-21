use anyhow::Result;

use crate::common;

#[tokio::test]
pub async fn code_received() -> Result<()> {
    let server = common::setup()?;
    let response = server.get("/v1/auth/provider/github")
        .add_query_param("code", "ARBITRARY_AUTHORIZATION_CODE")
        .await;

    response.assert_status_in_range(200..=201); // Can authorize either an existing or a new user

    let access_token = response.cookie("access_token");
    let refresh_token = response.cookie("refresh_token");

    assert_eq!(access_token.max_age().unwrap().whole_minutes(), 15);
    assert_eq!(refresh_token.max_age().unwrap().whole_days(), 1);

    Ok(())
}

#[tokio::test]
pub async fn code_missing() -> Result<()> {
    let server = common::setup()?;
    let response = server.get("/v1/auth/provider/github").await;

    response.assert_status_bad_request();

    Ok(())
}

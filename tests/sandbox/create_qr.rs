use chrono::{Duration, Utc};
use maib_client::models::request::CreateQR;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::common;

#[tokio::test]
pub async fn should_create_qr() {
    let (client, token) = common::setup();
    let expires_at = Utc::now() + Duration::days(4);
    let tz = chrono_tz::Tz::from_str("Europe/Chisinau").unwrap();
    let expires_at = expires_at.with_timezone(&tz).to_rfc3339();

    let fixed = CreateQR::new_dynamic_with_fixed_amount(
        Decimal::from(100),
        &expires_at,
        "foobar".to_owned(),
        "".to_owned(),
        "".to_owned(),
    );

    let result = client.create_qr(&fixed, &token).await;

    println!("{result:?}");

    assert!(result.is_ok());
}

use std::str::FromStr;

use chrono::{Duration, Utc};
use maib_client::models::request::CreateQR;
use rust_decimal::Decimal;

use crate::common;

#[tokio::test]
pub async fn should_get_payment_detail() {
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

    let result = client.create_qr(fixed, &token).await.unwrap();
    let pay_id = common::simulate_payment(&result.qr_id, Decimal::from(100), &token).await;

    assert!(pay_id.is_ok());
    let pay_id = pay_id.unwrap();
    eprintln!("payment id: {pay_id}");

    let detail = client.get_payment(&pay_id, &token).await;
    eprintln!("{detail:?}");

    assert!(detail.is_ok());
}

use chrono::{Duration, Utc};
use maib_client::models::{
    QRStatus,
    request::{CancelQR, CreateQR},
};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::common;

#[tokio::test]
pub async fn should_cancel_qr() {
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

    assert!(result.is_ok());

    let response = result.unwrap();
    let reason = CancelQR {
        reason: "foobar".to_owned(),
    };
    let cancel_result = client.cancel_qr(&response.qr_id, &reason, &token).await;
    assert!(cancel_result.is_ok());

    let cancel_response = cancel_result.unwrap();
    assert_eq!(cancel_response.qr_id, response.qr_id);
    assert_eq!(cancel_response.status, QRStatus::Cancelled);
}

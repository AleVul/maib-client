use chrono::{Duration, Utc};
use maib_client::{
    client::Client,
    error::Result,
    models::{
        AccessToken, PaymentId, QRId,
        request::CreateQR,
        response::{self},
    },
};
use rust_decimal::Decimal;
use std::{env, str::FromStr};

pub fn base_url_path() -> String {
    let base_url_path = env::var("MAIB_SANDBOX_BASE_PATH").unwrap();
    return base_url_path;
}

pub fn setup() -> (Client, AccessToken) {
    let access_token = env::var("MAIB_SANDBOX_ACCESS_TOKEN").unwrap();

    return (Client::new(base_url_path()), AccessToken::new(access_token));
}

pub async fn create_fix_payment_qr() -> (Client, AccessToken, response::CreateQRResponse) {
    let (client, token) = setup();
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

    let response = client.create_qr(fixed, &token).await;

    return (client, token, response.unwrap());
}

pub async fn simulate_payment(
    id: &QRId,
    amount: Decimal,
    token: &AccessToken,
) -> Result<PaymentId> {
    use reqwest::header::{self, HeaderMap, HeaderValue};

    let base_path = base_url_path();
    let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );

    let value = format!("Bearer {}", token.as_str());
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&value).unwrap(),
    );

    let url = format!("{}{}", base_path, "/v2/mia/test-pay");
    let http_client = reqwest::Client::new();
    let res = http_client
        .request(reqwest::Method::POST, url)
        .headers(headers)
        .json(&serde_json::json!({
            "qrId": id,
            "amount": amount,
            "iban": "MD88AG000000011621810140",
            "currency": "MDL",
            "payerName": "John D."
        }))
        .send()
        .await
        .unwrap();

    if res.status().as_u16() > 399 {
        panic!("API returned error {}", res.status());
    }

    let payload: serde_json::Value = res.json().await.unwrap();
    let result_field = payload.get("result").unwrap();

    eprintln!("result field ---> {result_field}");
    let value = result_field.get("payId").unwrap().to_string();
    let len = value.len() - 1;
    let value = value[1..len].to_owned();
    eprintln!("pay id ---> {value}");

    return Ok(PaymentId::new(value));
}

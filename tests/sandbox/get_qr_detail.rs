use crate::common;

#[tokio::test]
pub async fn should_get_qr_detail() {
    let (client, token, response) = common::create_fix_payment_qr().await;

    let result = client.get_qr(&response.qr_id, &token).await;

    assert!(result.is_ok());
}

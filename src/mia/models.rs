use std::sync::Arc;

use rust_decimal::Decimal;
use sha2::Digest;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientId(String);

impl ClientId {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "ClientId([redacted])");
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientSecret(String);

impl ClientSecret {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for ClientSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "ClientSecret([redacted])");
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccessToken(pub(crate) String);

impl AccessToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// This might leak the value in logs!!!
    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }
}

impl core::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "AccessToken([redacted])");
    }
}

#[derive(Debug)]
pub struct AccessTokenDuration(core::time::Duration);

impl From<AccessTokenDuration> for core::time::Duration {
    fn from(value: AccessTokenDuration) -> Self {
        return value.0;
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QRId(String);

impl QRId {
    pub fn new(value: String) -> Self {
        return Self(value);
    }

    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }
}

impl std::cmp::PartialEq<str> for QRId {
    fn eq(&self, other: &str) -> bool {
        return self.0.eq(other);
    }
}

impl core::fmt::Display for QRId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Signature(String);

impl Signature {
    pub fn new(value: String) -> Self {
        return Self(value);
    }

    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }
}

impl core::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Signature([redacted])");
    }
}

/// Signature key provided by MAIB.
#[derive(Debug)]
pub struct SignatureKey(Arc<str>);

impl SignatureKey {
    pub fn as_str(&self) -> &str {
        return &self.0;
    }
}

impl From<String> for SignatureKey {
    fn from(value: String) -> Self {
        return Self(Arc::from(value));
    }
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ExtensionId(String);

impl ExtensionId {
    pub fn new(value: String) -> Self {
        return Self(value);
    }

    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }
}

impl core::fmt::Display for ExtensionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PaymentId(String);

impl PaymentId {
    pub fn new(value: String) -> Self {
        return Self(value);
    }

    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }
}

impl core::fmt::Display for PaymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentType {
    Fixed,
    Controlled,
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentStatus {
    Executed,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum TokenType {
    Bearer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QRType {
    /// QR payment that can be paid
    /// more than once.
    Static,

    /// QR payment that can be paid once.
    Dynamic,

    /// QR payment can pe paid more than once.
    ///
    /// This also allows to modify amount and expiration date
    /// while is considere valid payment.
    Hybrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Currency {
    MDL,
}

impl Currency {
    pub fn code(self) -> &'static str {
        match self {
            Currency::MDL => "MDL",
        }
    }

    pub fn minor_currency_unit(self) -> i32 {
        match self {
            Currency::MDL => 100,
        }
    }
}

impl core::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.code());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QRStatus {
    Active,
    Inactive,
    Expired,
    Paid,
    Cancelled,
}

impl core::fmt::Display for QRStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            QRStatus::Active => write!(f, "Active"),
            QRStatus::Inactive => write!(f, "Inactive"),
            QRStatus::Expired => write!(f, "Expired"),
            QRStatus::Paid => write!(f, "Paid"),
            QRStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub(crate) amount: Decimal,
    pub(crate) commission: Decimal,
    pub(crate) currency: Currency,
    pub(crate) executed_at: String,
    pub(crate) extension_id: ExtensionId,
    pub(crate) order_id: Option<String>,
    pub(crate) pay_id: PaymentId,
    pub(crate) payer_iban: String,
    pub(crate) payer_name: String,
    pub(crate) qr_id: QRId,
    pub(crate) qr_status: QRStatus,
    pub(crate) reference_id: String,
    pub(crate) terminal_id: Option<String>,
}

impl Notification {
    pub fn pay_id(&self) -> &PaymentId {
        &self.pay_id
    }
}

#[derive(Debug)]
pub struct ValidSignatureNotification(pub Notification);

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPayload {
    pub(crate) result: Notification,
    pub(crate) signature: Signature,
}

impl NotificationPayload {
    pub(crate) fn build_signature(&self, key: SignatureKey) -> Signature {
        use base64::prelude::*;

        let n = &self.result;
        let mut this_signature = format!(
            "{}:{}:{}:{}:{}",
            n.amount, n.commission, n.currency, n.executed_at, n.extension_id
        );

        if let Some(ref order_id) = n.order_id {
            this_signature = format!("{this_signature}:{order_id}");
        }

        this_signature = format!(
            "{this_signature}:{}:{}:{}:{}:{}:{}",
            n.pay_id, n.payer_iban, n.payer_name, n.qr_id, n.qr_status, n.reference_id
        );

        if let Some(ref terminal_id) = n.terminal_id {
            this_signature = format!("{this_signature}:{terminal_id}");
        }

        this_signature = format!("{this_signature}:{}", key.0);

        let sig_sha256 = sha2::Sha256::digest(&this_signature);
        let encoded = hex::encode(sig_sha256);
        let signature = Signature::new(BASE64_STANDARD.encode(encoded));

        return signature;
    }

    /// Attempt to validate signature with provided key.
    ///
    /// If it is not valid, this will return [None].
    pub fn validate_signature(self, key: SignatureKey) -> Option<ValidSignatureNotification> {
        let signature = self.build_signature(key);

        if signature.eq(&self.signature) {
            return Some(ValidSignatureNotification(self.result));
        }

        return None;
    }

    pub fn notification(&self) -> &Notification {
        &self.result
    }
}

pub mod request {
    use rust_decimal::Decimal;

    use super::{ClientId, ClientSecret, Currency, PaymentType, QRType};

    #[derive(Debug, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetAccessToken<'a> {
        pub client_id: &'a ClientId,
        pub client_secret: &'a ClientSecret,
    }

    #[derive(Debug, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateQR<'a> {
        pub r#type: super::QRType,
        /// Date time when Dynamic QR expires.
        ///
        /// Must be a valid ISO 8601-1:2019 value.
        pub expires_at: Option<&'a str>,
        pub amount_type: super::PaymentType,

        pub amount: rust_decimal::Decimal,
        pub amount_min: Option<rust_decimal::Decimal>,
        pub amount_max: Option<rust_decimal::Decimal>,

        pub currency: super::Currency,
        pub description: String,
        pub order_id: Option<&'a str>,
        pub callback_url: String,
        pub redirect_url: String,
        pub terminal_id: Option<String>,
    }

    impl<'a> CreateQR<'a> {
        pub fn new_dynamic_with_fixed_amount(
            amount: Decimal,
            expires_at: &'a str,
            description: String,
            callback_url: String,
            redirect_url: String,
        ) -> Self {
            return CreateQR {
                r#type: QRType::Dynamic,
                expires_at: Some(expires_at),
                amount_type: PaymentType::Fixed,
                amount,
                amount_min: None,
                amount_max: None,
                currency: Currency::MDL,
                description,
                order_id: None,
                callback_url,
                redirect_url,
                terminal_id: None,
            };
        }
    }

    #[derive(Debug, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CancelQR {
        pub reason: String,
    }

    #[derive(Debug, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RefundPayment {
        pub reason: String,
    }
}

pub mod response {
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;

    use super::{Currency, ExtensionId, PaymentId, PaymentStatus, QRId};

    #[derive(Debug, serde::Deserialize)]
    pub struct ApiResponse<R> {
        pub(crate) result: Option<R>,
        pub(crate) errors: Option<Vec<crate::error::ApiError>>,
    }

    impl<R> From<ApiResponse<R>> for core::result::Result<R, crate::error::Error> {
        fn from(value: ApiResponse<R>) -> Self {
            if let Some(value) = value.result {
                return Ok(value);
            }

            if let Some(value) = value.errors {
                return Err(crate::error::Error::Api(value));
            }

            panic!();
        }
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthToken {
        access_token: super::AccessToken,
        expires_in: u64,
        token_type: super::TokenType,
    }

    impl AuthToken {
        /// Access token lifetime in seconds.
        pub fn expires_in(&self) -> super::AccessTokenDuration {
            return super::AccessTokenDuration(core::time::Duration::from_secs(self.expires_in));
        }

        pub fn access_token(&self) -> &super::AccessToken {
            &self.access_token
        }

        pub fn take_access_token(self) -> super::AccessToken {
            self.access_token
        }

        pub fn token_type(&self) -> super::TokenType {
            self.token_type
        }
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateQRResponse {
        pub qr_id: super::QRId,
        pub order_id: Option<String>,
        pub r#type: super::QRType,
        pub url: String,
        pub expires_at: String,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetQRDetails {
        pub qr_id: super::QRId,
        pub order_id: Option<String>,
        pub status: super::QRStatus,
        pub r#type: super::QRType,
        pub url: String,
        pub amount_type: super::PaymentType,
        pub currency: super::Currency,
        pub amount: Decimal,
        pub amount_min: Option<Decimal>,
        pub amount_max: Option<Decimal>,
        pub description: String,
        pub callback_url: String,
        pub redirect_url: String,
        pub terminal_id: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub expires_at: DateTime<Utc>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CancelQR {
        pub qr_id: super::QRId,
        pub status: super::QRStatus,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PaymentDetails {
        pub pay_id: PaymentId,
        pub reference_id: String,
        pub qr_id: QRId,
        pub extension_id: Option<ExtensionId>,
        pub order_id: Option<String>,
        pub amount: Decimal,
        pub commission: Decimal,
        pub currency: Currency,
        pub description: String,
        pub payer_name: String,
        pub payer_iban: String,
        pub status: PaymentStatus,
        pub executed_at: String,
        pub refunded_at: Option<String>,
        pub terminal_id: Option<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RefundPayment {
        pub pay_id: PaymentId,
        pub status: PaymentStatus,
    }
}

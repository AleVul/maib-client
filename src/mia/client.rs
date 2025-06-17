use reqwest::StatusCode;

use crate::{
    error::{Error, Result},
    models::{
        AccessToken, ClientId, ClientSecret, PaymentId, QRId,
        request::{self, CancelQR, GetAccessToken, RefundPayment},
        response::{self, AuthToken},
    },
};

#[derive(Debug)]
pub struct Client {
    http_client: reqwest::Client,
    api_base_url: String,
}

impl Client {
    pub fn new(api_base_url: String) -> Self {
        return Self {
            http_client: reqwest::Client::new(),
            api_base_url,
        };
    }

    /// Attempt to fetch a new [AccessToken]
    pub async fn get_access_token(
        &self,
        id: &ClientId,
        secret: &ClientSecret,
    ) -> Result<AuthToken> {
        let body = GetAccessToken {
            client_id: id,
            client_secret: secret,
        };

        let input = SendRequestInput {
            method: reqwest::Method::POST,
            url: "/v2/auth/token",
            token: None,
            body: Some(body),
        };
        return self.send_request(input).await;
    }

    pub async fn create_qr<'a, 'b>(
        &'a self,
        payload: request::CreateQR<'b>,
        token: &'a AccessToken,
    ) -> Result<response::CreateQRResponse> {
        let input = SendRequestInput {
            method: reqwest::Method::POST,
            url: "/v2/mia/qr",
            token: Some(token),
            body: Some(payload),
        };
        return self.send_request(input).await;
    }

    pub async fn get_qr(
        &self,
        qr_id: &QRId,
        token: &AccessToken,
    ) -> Result<response::GetQRDetails> {
        let url = format!("/v2/mia/qr/{}", qr_id.as_str());
        let input: SendRequestInput<QRId> = SendRequestInput {
            method: reqwest::Method::GET,
            url: url.as_str(),
            token: Some(token),
            body: None,
        };

        return self.send_request(input).await;
    }

    pub async fn cancel_qr(
        &self,
        qr_id: &QRId,
        payload: &CancelQR,
        token: &AccessToken,
    ) -> Result<response::CancelQR> {
        let url = format!("/v2/mia/qr/{qr_id}/cancel");
        let input = SendRequestInput {
            method: reqwest::Method::POST,
            url: url.as_str(),
            token: Some(token),
            body: Some(payload),
        };

        return self.send_request(input).await;
    }

    pub async fn get_payment(
        &self,
        id: &PaymentId,
        token: &AccessToken,
    ) -> Result<response::PaymentDetails> {
        let url = format!("/v2/mia/payments/{id}");
        let input: SendRequestInput<()> = SendRequestInput {
            method: reqwest::Method::GET,
            url: url.as_str(),
            token: Some(token),
            body: None,
        };

        return self.send_request(input).await;
    }

    pub async fn refund_payment(
        &self,
        id: &PaymentId,
        reason: String,
        token: &AccessToken,
    ) -> Result<response::RefundPayment> {
        let url = format!("/v2/mia/payments/{id}/refund");
        let payload = RefundPayment { reason };

        let input = SendRequestInput {
            method: reqwest::Method::POST,
            url: &url,
            token: Some(token),
            body: Some(payload),
        };

        return self.send_request(input).await;
    }

    async fn send_request<'a, B, R>(&self, input: SendRequestInput<'a, B>) -> Result<R>
    where
        B: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        use reqwest::header::{self, HeaderMap, HeaderValue};

        let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/json").unwrap(),
        );

        if input.method != reqwest::Method::GET {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str("application/json").unwrap(),
            );
        }

        if let Some(token) = input.token {
            let value = format!("Bearer {}", token.as_str());
            headers.insert(
                header::AUTHORIZATION,
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        let url = format!("{}{}", &self.api_base_url, input.url);
        let mut req = self.http_client.request(input.method, url).headers(headers);

        if let Some(ref body) = input.body {
            req = req.json(body);
        }

        let res = req
            .send()
            .await
            .map_err(|err| Error::Http(format!("error sending request: {err}")))?;

        let status = res.status().as_u16();

        if res.status() == 401 {
            return Err(Error::Unauthorized);
        }

        if status >= 400 && status < 500 {
            if res.status() == StatusCode::UNAUTHORIZED {
                return Err(Error::Http(format!(
                    "we made a bad request, status: {}",
                    status
                )));
            }
        }

        let res: response::ApiResponse<R> = res
            .json()
            .await
            .map_err(|err| Error::Json(format!("error parsing response: {err}")))?;

        if res.result.is_some() {
            return Ok(res.result.unwrap());
        }

        return Err(Error::Api(res.errors.unwrap()));
    }
}

struct SendRequestInput<'a, B: serde::Serialize> {
    method: reqwest::Method,
    url: &'a str,
    token: Option<&'a AccessToken>,
    body: Option<B>,
}

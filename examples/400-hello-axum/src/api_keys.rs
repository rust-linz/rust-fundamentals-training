use axum::{extract::{FromRequest, RequestParts}, http, async_trait};

#[derive(Debug)]
pub struct ApiKey(pub String);

#[async_trait]
impl<B> FromRequest<B> for ApiKey where B: Send {
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(header) = req.headers().get("X-API-KEY") {
            let header_str = header.to_str().map_err(|_| (http::StatusCode::UNAUTHORIZED, "Invalid API key"))?;
            let api_key_utf8 = base64::decode(header_str).map_err(|_| (http::StatusCode::UNAUTHORIZED, "Invalid API key"))?;
            let api_key = String::from_utf8(api_key_utf8).map_err(|_| (http::StatusCode::UNAUTHORIZED, "Invalid API key"))?;
            Ok(ApiKey(api_key))
        } else {
            Err((http::StatusCode::UNAUTHORIZED, "Missing API key header"))
        }
    }
}

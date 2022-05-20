use {
    axum::{
        async_trait,
        extract::{rejection::JsonRejection, FromRequest, RequestParts},
        http::StatusCode,
        BoxError,
    },
    serde::de::DeserializeOwned,
    serde_json::{json, Value},
    std::borrow::Cow,
};

// We define our own `Json` extractor that customizes the error from `axum::Json`
pub struct JsonExtractor<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for JsonExtractor<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::Json`
    T: DeserializeOwned,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                // convert the error from `axum::Json` into whatever we want
                let (status, body): (_, Cow<'_, str>) = match rejection {
                    JsonRejection::JsonDataError(err) => (
                        StatusCode::BAD_REQUEST,
                        format!("Invalid JSON request: {}", err).into(),
                    ),
                    JsonRejection::MissingJsonContentType(err) => {
                        (StatusCode::BAD_REQUEST, err.to_string().into())
                    }

                    err => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unknown internal error: {}", err).into(),
                    ),
                };
                tracing::error!("{body}");
                Err((
                    status,
                    // we use `axum::Json` here to generate a JSON response
                    // body but you can use whatever response you want
                    axum::Json(json!({ "error": body })),
                ))
            }
        }
    }
}

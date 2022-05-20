use {
    crate::domain::seatalk::schema::SeatalkResponse,
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
    reqwest::Error as RequestError,
    serde_json::json,
    std::convert::Infallible,
    tower::BoxError,
};

#[derive(Debug)]
pub enum AppError {
    RequestError(RequestError),
    SeatalkError(SeatalkResponse),
}

impl From<RequestError> for AppError {
    fn from(inner: RequestError) -> Self {
        AppError::RequestError(inner)
    }
}

impl From<SeatalkResponse> for AppError {
    fn from(inner: SeatalkResponse) -> Self {
        AppError::SeatalkError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SeatalkError(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!(e.msg)),
            AppError::RequestError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!("Request to seatalk api is failed."),
            ),
        };
        let body = Json(json!({
            "code" : status.as_u16(),
            "error": error_message,
        }));
        tracing::error!("{error_message}");
        (status, body).into_response()
    }
}

pub async fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "code" : 408,
                "error" : "Uhh ohh, request time out",
            })),
        ));
    };
    if error.is::<tower::load_shed::error::Overloaded>() {
        return Ok((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "code" : 503,
                "error" : "Uhh ohh, service unavailable",
            })),
        ));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "code" : 500,
            "error" : "Uhh ohh, unhandled internal error",
        })),
    ))
}
